use std::{fmt, fs};

#[derive(Default)]
struct Disk {
    data: Vec<Option<u32>>,
    max_block_id: u32,
}

impl Disk {
    fn compress_files(&mut self) {
        for file_id in (0..=self.max_block_id).rev() {
            let file_position = self.find_file(file_id).unwrap();
            let file_len = self.calc_file_len(file_id, file_position);

            if let Some(free_space) = self.find_free_space(file_len) {
                if free_space > file_position {
                    continue;
                }

                self.data[free_space..free_space + file_len].fill(Some(file_id));
                self.data[file_position..file_position + file_len].fill(None);
            }
        }
    }

    fn compress_blocks(&mut self) {
        while let Some(first_free) = self.find_fragmented_space() {
            let last_block = self.find_last_block().unwrap();
            self.data.swap(first_free, last_block);
        }
    }

    fn find_last_block(&self) -> Option<usize> {
        self.data.iter().rposition(|c| c.is_some())
    }

    fn calc_checksum(&self) -> usize {
        let mut checksum = 0;

        for (i, c) in self.data.iter().enumerate() {
            if c.is_none() {
                continue;
            }

            checksum += i * c.unwrap() as usize;
        }

        checksum
    }

    fn find_file(&self, file_id: u32) -> Option<usize> {
        self.data.iter().position(|c| *c == Some(file_id))
    }

    fn calc_file_len(&self, file_id: u32, file_position: usize) -> usize {
        self.data[file_position..]
            .iter()
            .take_while(|c| **c == Some(file_id))
            .count()
    }

    fn find_free_space(&self, len: usize) -> Option<usize> {
        let mut free_space = 0;
        let mut free_space_start = 0;

        for (i, c) in self.data.iter().enumerate() {
            match c {
                None => {
                    free_space += 1;

                    if free_space == len {
                        return Some(free_space_start);
                    }
                }
                Some(_) => {
                    free_space = 0;
                    free_space_start = i + 1;
                }
            }
        }

        None
    }

    fn find_fragmented_space(&self) -> Option<usize> {
        let first_free = self.data.iter().position(|c| c.is_none())?;
        let non_contiguous_block = self.data[first_free + 1..].iter().any(|&c| c.is_some());

        if non_contiguous_block {
            return Some(first_free);
        }

        None
    }
}

impl Clone for Disk {
    fn clone(&self) -> Self {
        Disk {
            data: self.data.clone(),
            max_block_id: self.max_block_id,
        }
    }
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data = self
            .data
            .iter()
            .map(|e| match e {
                None => ".".to_string(),
                Some(v) => format!("[{}]", v),
            })
            .collect::<String>();

        write!(f, "{}", data)
    }
}

struct DiskMap {
    data: Vec<char>,
}

impl DiskMap {
    fn expand(&self) -> Disk {
        let mut block_id = 0;
        let mut result = Disk::default();

        for i in 0..self.data.len() {
            let is_block = i % 2 == 0;
            let n = self.data[i].to_digit(10).unwrap() as usize;

            let value = if is_block {
                let v = Some(block_id);
                block_id += 1;
                v
            } else {
                None
            };

            result.data.extend(vec![value; n]);
        }

        result.max_block_id = block_id - 1;
        result
    }
}

impl From<&str> for DiskMap {
    fn from(s: &str) -> Self {
        DiskMap {
            data: s.chars().collect(),
        }
    }
}

fn main() {
    let file_path = "./d9-disk-fragmenter/input.txt";
    let contents = fs::read_to_string(file_path).unwrap().trim().to_string();
    let disk = DiskMap::from(contents.as_str()).expand();

    part1(disk.clone());
    part2(disk);
}

fn part1(mut disk: Disk) {
    disk.compress_blocks();
    println!("Solution (Part 1): {}", disk.calc_checksum());
}

fn part2(mut disk: Disk) {
    disk.compress_files();
    println!("Solution (Part 2): {}", disk.calc_checksum());
}
