use std::fs;

struct XmasWordPuzzle {
    grid: Grid,
}

impl XmasWordPuzzle {
    const WORD_XMAS: &'static str = "XMAS";

    fn new(grid: Grid) -> Self {
        XmasWordPuzzle { grid }
    }

    fn count_x_mas(&self) -> usize {
        self.coordinate_iter()
            .map(|(x, y)| self.count_x_mas_at(x, y))
            .sum()
    }

    fn count_xmas(&self) -> usize {
        self.coordinate_iter()
            .map(|(x, y)| self.count_xmas_at(x, y))
            .sum()
    }

    fn coordinate_iter(&self) -> impl Iterator<Item = (usize, usize)> + use<'_> {
        let Grid { width, height, .. } = &self.grid;
        (0..*height).flat_map(move |y| (0..*width).map(move |x| (x, y)))
    }

    fn count_x_mas_at(&self, x: usize, y: usize) -> usize {
        todo!()
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

        let has_horizontal_space = x + tail * dx >= 0 && x + tail * dx < self.grid.width as i32;
        let has_vertical_space = y + tail * dy >= 0 && y + tail * dy < self.grid.height as i32;

        if !has_horizontal_space || !has_vertical_space {
            return false;
        }

        for c in word.chars() {
            if self.grid.data[y as usize][x as usize] != c {
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
        self.find_str(x, y, 1, 1, word)
    }

    fn find_bottom_right_diagonal(&self, x: usize, y: usize, word: &str) -> bool {
        self.find_str(x, y, 1, -1, word)
    }

    fn find_top_left_diagonal(&self, x: usize, y: usize, word: &str) -> bool {
        self.find_str(x, y, -1, 1, word)
    }

    fn find_bottom_left_diagonal(&self, x: usize, y: usize, word: &str) -> bool {
        self.find_str(x, y, -1, -1, word)
    }
}

struct Grid {
    width: usize,
    height: usize,
    data: Vec<Vec<char>>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            height,
            data: vec![vec!['?'; width]; height],
        }
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

fn main() {
    let file_path = "./d4-ceres-search/input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let grid = Grid::from(contents.as_str());
    let puzzle = XmasWordPuzzle::new(grid);

    println!("Solution (Part 1): {}", puzzle.count_xmas());
    // println!("Solution (Part 2): {}", puzzle.count_x_mas());
}
