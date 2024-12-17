use std::fmt::{Display, Formatter};
use std::{cmp, fmt};

pub struct Grid {
    width: usize,
    height: usize,
    data: Vec<Vec<char>>,
}

impl Grid {
    const DEFAULT_CHAR: char = '?';

    pub fn new(width: usize, height: usize) -> Self {
        Grid::new_with_default_char(width, height, Self::DEFAULT_CHAR)
    }

    pub fn new_with_default_char(width: usize, height: usize, default_char: char) -> Self {
        Grid {
            width,
            height,
            data: vec![vec![default_char; width]; height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn size(&self) -> usize {
        self.width * self.height
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= 0.0 && x < self.width as f32 && y >= 0.0 && y < self.height as f32
    }

    pub fn get(&self, x: usize, y: usize) -> char {
        self.data[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, value: char) {
        self.data[y][x] = value;
    }

    pub fn coordinates_iter(&self) -> impl Iterator<Item = (usize, usize)> + use<'_> {
        (0..self.height()).flat_map(move |y| (0..self.width()).map(move |x| (x, y)))
    }

    pub fn iter(&self) -> impl Iterator<Item = &char> {
        self.data.iter().flat_map(|row| row.iter())
    }

    pub fn print_slice(&self, x: usize, y: usize, padding: usize) {
        let start_x = cmp::max(x as i32 - padding as i32, 0) as usize;
        let start_y = cmp::max(y as i32 - padding as i32, 0) as usize;
        let end_x = cmp::min(x + padding, self.width() - 1);
        let end_y = cmp::min(y + padding, self.height() - 1);

        let cell_width = 3;

        for cy in start_y..=end_y {
            for cx in start_x..=end_x {
                let is_current_position = cx == x && cy == y;

                if is_current_position {
                    print!(
                        "{:^width$}",
                        format!("[{}]", self.get(cx, cy)),
                        width = cell_width
                    );
                } else {
                    print!("{:^width$}", self.get(cx, cy), width = cell_width);
                }
            }

            println!();
        }
    }
}

impl Clone for Grid {
    fn clone(&self) -> Self {
        let mut new_grid = Grid::new(self.width(), self.height());

        for y in 0..self.height() {
            for x in 0..self.width() {
                new_grid.set(x, y, self.get(x, y));
            }
        }

        new_grid
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            for c in row {
                write!(f, "{}", c)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let mut grid = Grid::new(width, height);

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                grid.data[y][x] = c;
            }
        }

        grid
    }
}
