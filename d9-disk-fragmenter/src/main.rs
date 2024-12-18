use std::fs;

#[derive(Default)]
struct Disk {
    data: Vec<Option<u32>>,
}

impl Disk {
    fn compress(&mut self) {
        while let Some(first_free) = self.find_fragmented_space() {
            let last_block = self.data.iter().rposition(|c| c.is_some()).unwrap();
            self.data.swap(first_free, last_block);
        }
    }

    fn calc_checksum(&self) -> usize {
        let data = self.data.iter().flatten().collect::<Vec<&u32>>();
        let mut checksum = 0;

        for (i, c) in data.iter().enumerate() {
            checksum += i * **c as usize;
        }

        checksum
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
    let mut disk = DiskMap::from(contents.as_str()).expand();

    disk.compress();

    println!("Checksum: {}", disk.calc_checksum());
}
