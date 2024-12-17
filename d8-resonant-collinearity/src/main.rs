use bon::Builder;
use euclid::Point2D;
use itertools::Itertools;
use std::{collections::HashSet, fs};

use aoc24::grid::Grid;

type Point2d = Point2D<f32, f32>;

#[derive(Builder)]
struct FrequencyAnalyzerOptions {
    has_resonant_harmonics_detector: bool,
}

struct FrequencyAnalyzer {
    frequency_grid: Grid,
    anti_nodes_grid: Grid,
    options: FrequencyAnalyzerOptions,
}

impl FrequencyAnalyzer {
    const DEFAULT_GRID_CHAR: char = '.';
    const ANTI_NODE: char = '#';

    fn new(grid: Grid) -> Self {
        let options = FrequencyAnalyzerOptions {
            has_resonant_harmonics_detector: false,
        };

        Self::_new(grid, options)
    }

    fn with_options(grid: Grid, options: FrequencyAnalyzerOptions) -> Self {
        Self::_new(grid, options)
    }

    fn _new(grid: Grid, options: FrequencyAnalyzerOptions) -> Self {
        let width = grid.width();
        let height = grid.height();

        FrequencyAnalyzer {
            frequency_grid: grid,
            anti_nodes_grid: Grid::new_with_default_char(width, height, Self::DEFAULT_GRID_CHAR),
            options,
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
        for anti_node in self.find_anti_nodes(p1, p2) {
            self.anti_nodes_grid
                .set(anti_node.x as usize, anti_node.y as usize, Self::ANTI_NODE);
        }
    }

    fn find_anti_nodes(&self, p1: Point2d, p2: Point2d) -> Vec<Point2d> {
        let direction = (p2 - p1).normalize();

        if !self.options.has_resonant_harmonics_detector {
            let distance = p1.distance_to(p2);
            return self.filter_anti_nodes(&[p1 - direction * distance, p2 + direction * distance]);
        }

        let mut distance = 1.0;
        let mut anti_nodes = vec![];

        let mut debug_grid = self.frequency_grid.clone();
        debug_grid.set(p1.x as usize, p2.y as usize, 'X');
        debug_grid.set(p2.x as usize, p2.y as usize, 'Y');

        println!("{}", debug_grid);

        loop {
            let nodes = [p1 - direction * distance, p2 + direction * distance];
            let nodes = self.filter_anti_nodes(&nodes);

            if nodes.is_empty() {
                break;
            }

            anti_nodes.extend(nodes);
            distance += 1.0;
        }

        anti_nodes
    }

    fn filter_anti_nodes(&self, anti_nodes: &[Point2d]) -> Vec<Point2d> {
        anti_nodes
            .iter()
            .filter(|node| self.anti_nodes_grid.contains(node.x, node.y))
            .copied()
            .collect()
    }
}

// fn main() {
//     let file_path = "./d8-resonant-collinearity/input_example.txt";
//     let contents = fs::read_to_string(file_path).unwrap();
//     let grid = Grid::from(contents.as_str());

//     let mut analyzer = FrequencyAnalyzer::new(grid);

//     let num_anti_nodes = analyzer.analyze();
//     println!("Solution: {}", num_anti_nodes);
// }

fn main() {
    let file_path = "./d8-resonant-collinearity/input_example.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let grid = Grid::from(contents.as_str());

    let options = FrequencyAnalyzerOptions::builder()
        .has_resonant_harmonics_detector(true)
        .build();

    let mut analyzer = FrequencyAnalyzer::with_options(grid, options);

    let num_anti_nodes = analyzer.analyze();

    println!("{}", analyzer.anti_nodes_grid);
    println!("Solution (Part 2): {}", num_anti_nodes);
}
