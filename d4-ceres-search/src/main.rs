use aoc24::grid::Grid;
use std::fs;

struct XmasWordPuzzle {
    grid: Grid,
}

impl XmasWordPuzzle {
    const WORD_XMAS: &'static str = "XMAS";
    const WORD_MAS: &'static str = "MAS";

    fn new(grid: Grid) -> Self {
        XmasWordPuzzle { grid }
    }

    fn count_x_mas(&self) -> usize {
        self.grid
            .coordinates_iter()
            .map(|(x, y)| self.count_x_mas_at(x, y))
            .sum()
    }

    fn count_xmas(&self) -> usize {
        self.grid
            .coordinates_iter()
            .map(|(x, y)| self.count_xmas_at(x, y))
            .sum()
    }

    fn count_x_mas_at(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        let word = Self::WORD_MAS;
        let word_reversed = word.chars().rev().collect::<String>();

        let has_top_left = x as i32 > 0 && y as i32 > 0;
        let has_bottom_left = x as i32 > 0 && y + 1 < self.grid.height();

        // println!("({}, {})", x, y);
        // self.print_slice(x, y, 10);

        let has_diagonal_1 = has_top_left
            && (self.find_bottom_right_diagonal(x - 1, y - 1, word)
                || self.find_bottom_right_diagonal(x - 1, y - 1, &word_reversed));

        let has_diagonal_2 = has_bottom_left
            && (self.find_top_right_diagonal(x - 1, y + 1, word)
                || self.find_top_right_diagonal(x - 1, y + 1, &word_reversed));

        if has_diagonal_1 && has_diagonal_2 {
            count += 1;
        }

        count
    }

    fn count_xmas_at(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        if self.find_left_to_right(x, y, Self::WORD_XMAS) {
            count += 1;
        }

        if self.find_right_to_left(x, y, Self::WORD_XMAS) {
            count += 1;
        }

        if self.find_top_to_bottom(x, y, Self::WORD_XMAS) {
            count += 1;
        }

        if self.find_bottom_to_top(x, y, Self::WORD_XMAS) {
            count += 1;
        }

        if self.find_top_right_diagonal(x, y, Self::WORD_XMAS) {
            count += 1;
        }

        if self.find_bottom_right_diagonal(x, y, Self::WORD_XMAS) {
            count += 1;
        }

        if self.find_top_left_diagonal(x, y, Self::WORD_XMAS) {
            count += 1;
        }

        if self.find_bottom_left_diagonal(x, y, Self::WORD_XMAS) {
            count += 1;
        }

        count
    }

    fn find_str(&self, x: usize, y: usize, dx: i32, dy: i32, word: &str) -> bool {
        let mut x = x as i32;
        let mut y = y as i32;

        let word_len = word.len() as i32;
        let tail = word_len - 1;

        let has_horizontal_space = x + tail * dx >= 0 && x + tail * dx < self.grid.width() as i32;
        let has_vertical_space = y + tail * dy >= 0 && y + tail * dy < self.grid.height() as i32;

        if !has_horizontal_space || !has_vertical_space {
            return false;
        }

        for c in word.chars() {
            if self.grid.get(x as usize, y as usize) != c {
                return false;
            }

            x += dx;
            y += dy;
        }

        true
    }

    fn find_left_to_right(&self, x: usize, y: usize, word: &str) -> bool {
        self.find_str(x, y, 1, 0, word)
    }

    fn find_right_to_left(&self, x: usize, y: usize, word: &str) -> bool {
        self.find_str(x, y, -1, 0, word)
    }

    fn find_top_to_bottom(&self, x: usize, y: usize, word: &str) -> bool {
        self.find_str(x, y, 0, 1, word)
    }

    fn find_bottom_to_top(&self, x: usize, y: usize, word: &str) -> bool {
        self.find_str(x, y, 0, -1, word)
    }

    fn find_top_right_diagonal(&self, x: usize, y: usize, word: &str) -> bool {
        self.find_str(x, y, 1, -1, word)
    }

    fn find_bottom_right_diagonal(&self, x: usize, y: usize, word: &str) -> bool {
        self.find_str(x, y, 1, 1, word)
    }

    fn find_top_left_diagonal(&self, x: usize, y: usize, word: &str) -> bool {
        self.find_str(x, y, -1, -1, word)
    }

    fn find_bottom_left_diagonal(&self, x: usize, y: usize, word: &str) -> bool {
        self.find_str(x, y, -1, 1, word)
    }
}

fn main() {
    let file_path = "./d4-ceres-search/input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let grid = Grid::from(contents.as_str());
    let puzzle = XmasWordPuzzle::new(grid);

    println!("Solution (Part 1): {}", puzzle.count_xmas());
    println!("Solution (Part 2): {}", puzzle.count_x_mas());
}
