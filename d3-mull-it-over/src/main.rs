use std::fs;

use regex::Regex;

struct Processor {
    accumulator: i32,
    is_mul_enabled: bool,
}

impl Processor {
    const INSTRUCTION_MUL: &'static str = "mul";
    const INSTRUCTION_DO: &'static str = "do";
    const INSTRUCTION_DONT: &'static str = "don't";

    fn new() -> Processor {
        Processor {
            accumulator: 0,
            is_mul_enabled: true,
        }
    }

    fn execute_instruction(&mut self, instruction: &str) -> Result<(), String> {
        let open_paren_index = instruction.find("(").unwrap();
        let function_name = &instruction[0..open_paren_index];
        let parameters = &instruction[open_paren_index + 1..instruction.find(")").unwrap()];

        match function_name {
            Self::INSTRUCTION_MUL => self.execute_mul(parameters),
            Self::INSTRUCTION_DO => self.is_mul_enabled = true,
            Self::INSTRUCTION_DONT => self.is_mul_enabled = false,
            _ => return Err(format!("Invalid instruction: {}", instruction)),
        }

        Ok(())
    }

    fn execute_mul(&mut self, parameters: &str) {
        if !self.is_mul_enabled {
            return;
        }

        let parameters = parameters
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        self.accumulator += parameters[0] * parameters[1];
    }
}

fn main() {
    let file_path = "./d3-mull-it-over/input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let pattern = r"(don't\(\))|(do\(\))|(mul\(\d{1,3},\d{1,3}\))";
    let regex = Regex::new(pattern).unwrap();
    let mut processor = Processor::new();

    for instruction in regex.find_iter(&contents) {
        processor.execute_instruction(instruction.as_str()).unwrap();
    }

    println!("Total: {}", processor.accumulator);
}
