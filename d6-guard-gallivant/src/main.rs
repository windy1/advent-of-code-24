use std::{collections::HashSet, fs};

use aoc24::grid::Grid;

struct PatrolSimulator {
    grid: Grid,
    guard_position: (usize, usize),
    history: HashSet<(usize, usize)>,
}

impl PatrolSimulator {
    const GUARD_NORTH: char = '^';
    const GUARD_SOUTH: char = 'v';
    const GUARD_WEST: char = '<';
    const GUARD_EAST: char = '>';
    const OBSTACLE: char = '#';

    const GUARD_CHARS: [char; 4] = [
        Self::GUARD_NORTH,
        Self::GUARD_EAST,
        Self::GUARD_SOUTH,
        Self::GUARD_WEST,
    ];

    fn new(grid: Grid) -> Self {
        PatrolSimulator {
            grid,
            guard_position: (0, 0),
            history: HashSet::new(),
        }
    }

    fn simulate(&mut self) -> usize {
        self.guard_position = self.find_guard_position();

        loop {
            // println!("{}", self.grid);

            self.history.insert(self.guard_position);

            let (nx, ny) = match self.get_next_position() {
                Some(pos) => pos,
                None => break,
            };

            if self.grid.get(nx, ny) == Self::OBSTACLE {
                self.rotate_guard();
                continue;
            }

            self.move_guard(nx, ny);
        }

        self.history.len()
    }

    fn move_guard(&mut self, x: usize, y: usize) {
        let (px, py) = self.guard_position;
        let guard_char = self.grid.get(px, py);
        self.grid.set(px, py, '.');
        self.grid.set(x, y, guard_char);
        self.guard_position = (x, y);
    }

    fn rotate_guard(&mut self) {
        let (gx, gy) = self.guard_position;
        let guard_char = self.grid.get(gx, gy);

        let rotation_index = Self::GUARD_CHARS
            .iter()
            .position(|&c| c == guard_char)
            .unwrap();

        let next_char = Self::GUARD_CHARS[(rotation_index + 1) % Self::GUARD_CHARS.len()];
        self.grid.set(gx, gy, next_char)
    }

    fn find_guard_position(&self) -> (usize, usize) {
        self.grid
            .coordinates_iter()
            .find(|(x, y)| Self::GUARD_CHARS.contains(&self.grid.get(*x, *y)))
            .unwrap()
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
    let mut sim = PatrolSimulator::new(grid);
    let distinct_positions = sim.simulate();

    println!("Solution: {}", distinct_positions);
}
