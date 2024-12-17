use std::fs;

use aoc24::grid::Grid;

struct PatrolSimulator {
    grid: Grid,
    guard_position: (usize, usize),
}

enum Simulation {
    DistinctPositions(usize),
    LoopDetected,
}

impl PatrolSimulator {
    const GUARD_NORTH: char = '^';
    const GUARD_SOUTH: char = 'v';
    const GUARD_WEST: char = '<';
    const GUARD_EAST: char = '>';
    const OBSTACLES: [char; 2] = ['#', 'O'];
    const PATH_TURN: char = '+';
    const PATH_VERTICAL: char = '|';
    const PATH_HORIZONTAL: char = '-';

    const PATH_CHARS: [char; 3] = [Self::PATH_TURN, Self::PATH_VERTICAL, Self::PATH_HORIZONTAL];

    const GUARD_CHARS: [char; 4] = [
        Self::GUARD_NORTH,
        Self::GUARD_EAST,
        Self::GUARD_SOUTH,
        Self::GUARD_WEST,
    ];

    fn new(grid: Grid) -> Result<Self, String> {
        let mut sim = PatrolSimulator {
            grid,
            guard_position: (0, 0),
        };

        sim.guard_position = sim.find_guard_position()?;

        Ok(sim)
    }

    fn simulate(&mut self) -> Result<Simulation, String> {
        let mut rotated_last_turn = false;
        let mut previous_char: Option<char> = None;

        while let Some((nx, ny)) = self.get_next_position() {
            let next_char = self.grid.get(nx, ny);

            if Self::OBSTACLES.contains(&next_char) {
                self.rotate_guard()?;
                rotated_last_turn = true;
                continue;
            }

            if rotated_last_turn && previous_char.map(|c| c == Self::PATH_TURN).unwrap_or(false) {
                return Ok(Simulation::LoopDetected);
            }

            self.move_guard(nx, ny, rotated_last_turn)?;

            rotated_last_turn = false;
            previous_char = Some(next_char);
        }

        Ok(Simulation::DistinctPositions(self.path_len()))
    }

    fn path_len(&self) -> usize {
        self.grid
            .iter()
            .filter(|&c| Self::PATH_CHARS.contains(c) || Self::GUARD_CHARS.contains(c))
            .count()
    }

    fn move_guard(&mut self, x: usize, y: usize, rotated_last_turn: bool) -> Result<(), String> {
        let (px, py) = self.guard_position;
        let guard_char = self.grid.get(px, py);
        let path_char = self.get_path_character(guard_char, rotated_last_turn)?;

        self.grid.set(px, py, path_char);
        self.grid.set(x, y, guard_char);
        self.guard_position = (x, y);

        Ok(())
    }

    fn get_path_character(
        &self,
        guard_char: char,
        rotated_last_turn: bool,
    ) -> Result<char, String> {
        if rotated_last_turn {
            return Ok(Self::PATH_TURN);
        }

        match guard_char {
            Self::GUARD_NORTH | Self::GUARD_SOUTH => Ok(Self::PATH_VERTICAL),
            Self::GUARD_EAST | Self::GUARD_WEST => Ok(Self::PATH_HORIZONTAL),
            _ => Err("Invalid guard character".into()),
        }
    }

    fn rotate_guard(&mut self) -> Result<(), String> {
        let (gx, gy) = self.guard_position;
        let guard_char = self.grid.get(gx, gy);

        let rotation_index = Self::GUARD_CHARS
            .iter()
            .position(|&c| c == guard_char)
            .ok_or_else(|| "Invalid guard character".to_string())?;

        let next_char = Self::GUARD_CHARS[(rotation_index + 1) % Self::GUARD_CHARS.len()];
        self.grid.set(gx, gy, next_char);

        Ok(())
    }

    fn find_guard_position(&self) -> Result<(usize, usize), String> {
        self.grid
            .coordinates_iter()
            .find(|(x, y)| Self::GUARD_CHARS.contains(&self.grid.get(*x, *y)))
            .ok_or_else(|| "Guard position not found".to_string())
    }

    fn get_next_position(&self) -> Option<(usize, usize)> {
        let mut dx: i32 = 0;
        let mut dy: i32 = 0;
        let guard_char = self.grid.get(self.guard_position.0, self.guard_position.1);
        let (gx, gy) = (self.guard_position.0 as i32, self.guard_position.1 as i32);

        match guard_char {
            Self::GUARD_NORTH => dy = -1,
            Self::GUARD_SOUTH => dy = 1,
            Self::GUARD_WEST => dx = -1,
            Self::GUARD_EAST => dx = 1,
            _ => return None,
        }

        let (nx, ny) = (gx + dx, gy + dy);

        if (nx < 0 || nx >= self.grid.width() as i32) || (ny < 0 || ny >= self.grid.height() as i32)
        {
            return None;
        }

        Some((nx as usize, ny as usize))
    }
}

fn main() {
    let file_path = "./d6-guard-gallivant/input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let grid = Grid::from(contents.as_str());

    let solution_p1 = part1(grid.clone());
    let solution_p2 = part2(grid);

    println!("Solution (Part 1): {}", solution_p1);
    println!("Solution (Part 2): {}", solution_p2);
}

fn part1(grid: Grid) -> usize {
    let mut sim = PatrolSimulator::new(grid).unwrap();

    match sim.simulate().unwrap() {
        Simulation::DistinctPositions(distinct_positions) => distinct_positions,
        Simulation::LoopDetected => {
            panic!("Loop detected");
        }
    }
}

fn part2(grid: Grid) -> usize {
    let mut possible_obstacles = 0;

    for (i, (x, y)) in grid.coordinates_iter().enumerate() {
        println!(
            "Checking ({}, {}) ({:.1}%)",
            x,
            y,
            i as f32 * 100.0 / grid.size() as f32
        );

        let grid_clone = grid.clone();
        let mut sim = PatrolSimulator::new(grid_clone).unwrap();

        if sim.guard_position == (x, y) {
            continue;
        }

        sim.grid.set(x, y, 'O');

        if let Simulation::LoopDetected = sim.simulate().unwrap() {
            possible_obstacles += 1;
        }
    }

    possible_obstacles
}
