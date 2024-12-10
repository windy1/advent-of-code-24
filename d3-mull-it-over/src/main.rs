use std::fs;

use regex::Regex;

fn main() {
    let file_path = "./d3-mull-it-over/input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let pattern = r"(don't\(\))|(do\(\))|(mul\(\d{1,3},\d{1,3}\))";
    let regex = Regex::new(pattern).unwrap();

    let total = regex
        .find_iter(&contents)
        .map(|instruction| execute_instruction(instruction.as_str()))
        .sum::<i32>();

    println!("Total: {}", total);
}

fn execute_instruction(instruction: &str) -> i32 {
    println!("{}", instruction);

    let parameters = instruction
        [instruction.find("(").unwrap() + 1..instruction.find(")").unwrap()]
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    parameters[0] * parameters[1]
}
