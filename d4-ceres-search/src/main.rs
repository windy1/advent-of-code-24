use std::fs;

struct XmasWordPuzzle {
    grid: Grid,
}

impl XmasWordPuzzle {
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

        if self.check_left_to_right(x, y) {
            count += 1;
        }

        if self.check_right_to_left(x, y) {
            count += 1;
        }

        if self.check_top_to_bottom(x, y) {
            count += 1;
        }

        if self.check_bottom_to_top(x, y) {
            count += 1;
        }

        if self.check_top_right_diagonal(x, y) {
            count += 1;
        }

        if self.check_bottom_right_diagonal(x, y) {
            count += 1;
        }

        if self.check_top_left_diagonal(x, y) {
            count += 1;
        }

        if self.check_bottom_left_diagonal(x, y) {
            count += 1;
        }

        count
    }

    fn find_str(&self, x: usize, y: usize, dx: i32, dy: i32, word: &str) -> bool {
        let mut x = x as i32;
        let mut y = y as i32;
        let word_len = word.len() as i32;

        let has_horizontal_space =
            x + word_len * dx >= 0 && x + word_len * dx < self.grid.width as i32;

        if !has_horizontal_space {
            return false;
        }

        todo!()
    }

    fn check_left_to_right(&self, x: usize, y: usize) -> bool {
        if x + 3 >= self.grid.width {
            return false;
        }

        self.grid.data[y][x] == 'X'
            && self.grid.data[y][x + 1] == 'M'
            && self.grid.data[y][x + 2] == 'A'
            && self.grid.data[y][x + 3] == 'S'
    }

    fn check_right_to_left(&self, x: usize, y: usize) -> bool {
        if x < 3 {
            return false;
        }

        self.grid.data[y][x] == 'X'
            && self.grid.data[y][x - 1] == 'M'
            && self.grid.data[y][x - 2] == 'A'
            && self.grid.data[y][x - 3] == 'S'
    }

    fn check_top_to_bottom(&self, x: usize, y: usize) -> bool {
        if y + 3 >= self.grid.height {
            return false;
        }

        self.grid.data[y][x] == 'X'
            && self.grid.data[y + 1][x] == 'M'
            && self.grid.data[y + 2][x] == 'A'
            && self.grid.data[y + 3][x] == 'S'
    }

    fn check_bottom_to_top(&self, x: usize, y: usize) -> bool {
        if y < 3 {
            return false;
        }

        self.grid.data[y][x] == 'X'
            && self.grid.data[y - 1][x] == 'M'
            && self.grid.data[y - 2][x] == 'A'
            && self.grid.data[y - 3][x] == 'S'
    }

    fn check_top_right_diagonal(&self, x: usize, y: usize) -> bool {
        if x + 3 >= self.grid.width || y + 3 >= self.grid.height {
            return false;
        }

        self.grid.data[y][x] == 'X'
            && self.grid.data[y + 1][x + 1] == 'M'
            && self.grid.data[y + 2][x + 2] == 'A'
            && self.grid.data[y + 3][x + 3] == 'S'
    }

    fn check_bottom_right_diagonal(&self, x: usize, y: usize) -> bool {
        if x + 3 >= self.grid.width || y < 3 {
            return false;
        }

        self.grid.data[y][x] == 'X'
            && self.grid.data[y - 1][x + 1] == 'M'
            && self.grid.data[y - 2][x + 2] == 'A'
            && self.grid.data[y - 3][x + 3] == 'S'
    }

    fn check_top_left_diagonal(&self, x: usize, y: usize) -> bool {
        if x < 3 || y + 3 >= self.grid.height {
            return false;
        }

        self.grid.data[y][x] == 'X'
            && self.grid.data[y + 1][x - 1] == 'M'
            && self.grid.data[y + 2][x - 2] == 'A'
            && self.grid.data[y + 3][x - 3] == 'S'
    }

    fn check_bottom_left_diagonal(&self, x: usize, y: usize) -> bool {
        if x < 3 || y < 3 {
            return false;
        }

        self.grid.data[y][x] == 'X'
            && self.grid.data[y - 1][x - 1] == 'M'
            && self.grid.data[y - 2][x - 2] == 'A'
            && self.grid.data[y - 3][x - 3] == 'S'
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
