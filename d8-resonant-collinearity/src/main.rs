use euclid::Point2D;
use itertools::Itertools;
use std::{collections::HashSet, fs};

use aoc24::grid::Grid;

type Point2d = Point2D<f32, f32>;

struct FrequencyAnalyzer {
    frequency_grid: Grid,
    anti_nodes_grid: Grid,
}

impl FrequencyAnalyzer {
    const DEFAULT_GRID_CHAR: char = '.';
    const ANTI_NODE: char = '#';

    fn new(grid: Grid) -> Self {
        let width = grid.width();
        let height = grid.height();

        FrequencyAnalyzer {
            frequency_grid: grid,
            anti_nodes_grid: Grid::new_with_default_char(width, height, Self::DEFAULT_GRID_CHAR),
        }
    }

    fn analyze(&mut self) -> usize {
        let distinct_frequencies = self
            .frequency_grid
            .iter()
            .filter(|c| c.is_alphanumeric())
            .collect::<HashSet<_>>();

        let instances = distinct_frequencies
            .iter()
            .map(|frequency| self.find_frequency_positions(frequency))
            .collect::<Vec<Vec<Point2d>>>();

        for instances_iter in instances {
            let coordinate_pairs = instances_iter.iter().combinations(2);

            for points in coordinate_pairs {
                self.set_anti_nodes(*points[0], *points[1]);
            }
        }

        self.anti_nodes_grid
            .iter()
            .filter(|c| **c == Self::ANTI_NODE)
            .count()
    }

    fn find_frequency_positions(&self, frequency: &char) -> Vec<Point2d> {
        self.frequency_grid
            .coordinates_iter()
            .filter(|(x, y)| self.frequency_grid.get(*x, *y) == *frequency)
            .map(|(x, y)| Point2d::new(x as f32, y as f32))
            .collect()
    }

    fn set_anti_nodes(&mut self, p1: Point2d, p2: Point2d) {
        let distance = p1.distance_to(p2);
        let direction = (p2 - p1).normalize();
        let anti_node1 = p1 - direction * distance;
        let anti_node2 = p2 + direction * distance;

        if self.anti_nodes_grid.contains(anti_node1.x, anti_node1.y) {
            self.anti_nodes_grid.set(
                anti_node1.x as usize,
                anti_node1.y as usize,
                Self::ANTI_NODE,
            );
        }

        if self.anti_nodes_grid.contains(anti_node2.x, anti_node2.y) {
            self.anti_nodes_grid.set(
                anti_node2.x as usize,
                anti_node2.y as usize,
                Self::ANTI_NODE,
            );
        }
    }
}

fn main() {
    let file_path = "./d8-resonant-collinearity/input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let grid = Grid::from(contents.as_str());

    let mut analyzer = FrequencyAnalyzer::new(grid);

    let num_anti_nodes = analyzer.analyze();
    println!("Solution: {}", num_anti_nodes);
}
