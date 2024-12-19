use std::{collections::HashMap, fs};

#[derive(Clone)]
struct MagicStones {
    data: HashMap<usize, usize>,
}

impl MagicStones {
    const MULTIPLIER: usize = 2024;

    fn blink(&mut self) {
        let mut new_data = HashMap::new();

        for (&stone, &count) in self.data.iter() {
            if stone == 0 {
                *new_data.entry(1).or_insert(0) += count;
                continue;
            }

            if let Some((left, right)) = self.split_stone(stone) {
                *new_data.entry(left).or_insert(0) += count;
                *new_data.entry(right).or_insert(0) += count;
                continue;
            }

            *new_data.entry(stone * Self::MULTIPLIER).or_insert(0) += count;
        }

        self.data = new_data;
    }

    fn len(&self) -> usize {
        self.data.values().sum()
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
        let stones = s
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let mut data = HashMap::new();

        for stone in stones {
            *data.entry(stone).or_insert(0) += 1;
        }

        MagicStones { data }
    }
}

fn main() {
    let file_path = "./d11-plutonian-pebbles/input.txt";
    let contents = fs::read_to_string(file_path).unwrap().trim().to_string();
    let magic_stones = MagicStones::from(contents.as_str());

    part1(magic_stones.clone());
    part2(magic_stones);
}

fn part1(mut magic_stones: MagicStones) {
    for i in 0..25 {
        magic_stones.blink();
        println!("Blink #{}: {}", i + 1, magic_stones.data.len());
    }

    println!("Solution (Part 1): {}", magic_stones.len());
}

fn part2(mut magic_stones: MagicStones) {
    for i in 0..75 {
        magic_stones.blink();
        println!("Blink #{}: {}", i + 1, magic_stones.data.len());
    }

    println!("Solution (Part 2): {}", magic_stones.len());
}
