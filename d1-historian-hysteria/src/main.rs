use std::fs;

struct Input {
    left: Vec<i32>,
    right: Vec<i32>,
}

fn main() {
    let file_path = "./d1-historian-hysteria/input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let mut input = parse_input(&contents);

    input.left.sort();
    input.right.sort();

    let distance = calc_distance(&input.left, &input.right);
    let similarity_score = calc_similarity_score(&input.left, &input.right);

    println!("Total distance: {}", distance);
    println!("Similarity score: {}", similarity_score);
}

fn calc_similarity_score(a: &[i32], b: &[i32]) -> i32 {
    let mut scores = Vec::new();

    for x in a {
        let occurrences = b.iter().filter(|&y| x == y).count() as i32;
        scores.push(x * occurrences);
    }

    scores.iter().sum()
}

fn calc_distance(a: &[i32], b: &[i32]) -> i32 {
    let mut distances = Vec::new();

    for i in 0..a.len() {
        let distance = (a[i] - b[i]).abs();
        distances.push(distance);
    }

    distances.iter().sum()
}

fn parse_input(str: &str) -> Input {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in str.lines() {
        let parts: Vec<&str> = line.split_whitespace().map(|x| x.trim()).collect();
        let left_elem = parts[0].parse::<i32>().unwrap();
        let right_elem = parts[1].parse::<i32>().unwrap();

        left.push(left_elem);
        right.push(right_elem);
    }

    Input { left, right }
}
