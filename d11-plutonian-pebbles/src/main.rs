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

            if let Some((left, right)) = self.split_stone(stone) {
                new_data.push(left);
                new_data.push(right);
                continue;
            }

            new_data.push(stone * Self::MULTIPLIER);
        }

        self.data = new_data;
    }

    fn split_stone(&self, stone: usize) -> Option<(usize, usize)> {
        let digits = (stone as f32).log10().floor() as usize + 1;

        if digits % 2 != 0 {
            return None;
        }

        let divisor = 10usize.pow(digits as u32 / 2);
        let left = stone / divisor;
        let right = stone % divisor;

        Some((left, right))
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

// fn main() {
//     let file_path = "./d11-plutonian-pebbles/input.txt";
//     let contents = fs::read_to_string(file_path).unwrap().trim().to_string();
//     let mut magic_stones = MagicStones::from(contents.as_str());

//     for i in 0..25 {
//         magic_stones.blink();
//         println!("Blink #{}: {}", i + 1, magic_stones.data.len());
//     }

//     println!("Solution: {}", magic_stones.data.len());
// }

fn main() {
    let file_path = "./d11-plutonian-pebbles/input.txt";
    let contents = fs::read_to_string(file_path).unwrap().trim().to_string();
    let mut magic_stones = MagicStones::from(contents.as_str());

    for i in 0..75 {
        magic_stones.blink();
        println!("Blink #{}: {}", i + 1, magic_stones.data.len());
    }

    println!("Solution: {}", magic_stones.data.len());
}
