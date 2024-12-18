use std::{collections::HashSet, fs};

use aoc24::grid::Grid;

struct TrailMap {
    data: Grid,
    distinct_mode: bool,
}

impl TrailMap {
    fn new(data: Grid) -> Self {
        TrailMap {
            data,
            distinct_mode: false,
        }
    }

    fn calc_score(&self) -> usize {
        self.find_trailheads()
            .iter()
            .map(|(x, y)| self.calc_trailhead_score(*x, *y, &mut HashSet::default()))
            .sum()
    }

    fn calc_trailhead_score(
        &self,
        x: usize,
        y: usize,
        visited: &mut HashSet<(usize, usize)>,
    ) -> usize {
        if !self.distinct_mode {
            if visited.contains(&(x, y)) {
                return 0;
            }

            visited.insert((x, y));
        }

        if self.get_height_at(x, y) == 9 {
            return 1;
        }

        let score: usize = self
            .traversable_neighbors(x, y)
            .into_iter()
            .map(|(nx, ny)| self.calc_trailhead_score(nx, ny, visited))
            .sum();

        score
    }

    fn traversable_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let base = self.get_height_at(x, y);

        self.data
            .neighbors_iter(x, y)
            .filter(|(nx, ny)| self.get_height_at(*nx, *ny) as i32 - base as i32 == 1)
            .collect()
    }

    fn get_height_at(&self, x: usize, y: usize) -> usize {
        self.data.get(x, y).to_digit(10).unwrap() as usize
    }

    fn find_trailheads(&self) -> Vec<(usize, usize)> {
        self.data
            .coordinates_iter()
            .filter(|(x, y)| self.data.get(*x, *y) == '0')
            .collect()
    }
}

fn main() {
    let file_path = "./d10-hoof-it/input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let mut map = TrailMap::new(Grid::from(contents.as_str()));

    println!("Solution (Part 1): {}", map.calc_score());

    map.distinct_mode = true;

    println!("Solution (Part 2): {}", map.calc_score());
}
