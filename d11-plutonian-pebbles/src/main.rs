use std::fs;

struct MagicStones {
    data: Vec<usize>,
}

impl MagicStones {
    const MULTIPLIER: usize = 2024;

    fn blink(&mut self) {
        let mut new_data = vec![];

        for &stone in self.data.iter() {
            if stone == 0 {
                new_data.push(1);
                continue;
            }

            let stone_str = stone.to_string();

            if stone_str.len() % 2 == 0 {
                let (left, right) = self.split_stone(stone_str.as_str());
                new_data.push(left);
                new_data.push(right);
                continue;
            }

            new_data.push(stone * Self::MULTIPLIER);
        }

        self.data = new_data;
    }

    fn split_stone(&self, stone_str: &str) -> (usize, usize) {
        let mid = stone_str.len() / 2;
        let left = stone_str[..mid].parse::<usize>().unwrap();
        let right = stone_str[mid..].parse::<usize>().unwrap();
        (left, right)
    }
}

impl From<&str> for MagicStones {
    fn from(s: &str) -> Self {
        MagicStones {
            data: s
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect(),
        }
    }
}

fn main() {
    let file_path = "./d11-plutonian-pebbles/input.txt";
    let contents = fs::read_to_string(file_path).unwrap().trim().to_string();
    let mut magic_stones = MagicStones::from(contents.as_str());

    for _ in 0..25 {
        magic_stones.blink();
    }

    println!("Solution: {}", magic_stones.data.len());
}
