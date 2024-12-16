pub struct Grid {
    width: usize,
    height: usize,
    data: Vec<Vec<char>>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            height,
            data: vec![vec!['?'; width]; height],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> char {
        self.data[y][x]
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
