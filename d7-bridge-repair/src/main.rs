use itertools::Itertools;
use std::fs;

struct Equation {
    test_value: i64,
    operands: Vec<i64>,
}

impl Equation {
    const OPERATOR_ADD: char = '+';
    const OPERATOR_MULTIPLY: char = '*';
    const OPERATORS: [char; 2] = [Self::OPERATOR_ADD, Self::OPERATOR_MULTIPLY];

    fn is_valid(&self) -> Result<bool, String> {
        let num_operators = self.operands.len() - 1;

        let operator_permutations = (0..num_operators)
            .map(|_| Self::OPERATORS.iter())
            .multi_cartesian_product();

        for operators in operator_permutations {
            if self.is_valid_with_operators(&operators)? {
                self.print_with_operators(&operators);
                return Ok(true);
            }
        }

        println!("{}: {:?} \u{2717}", self.test_value, self.operands);

        Ok(false)
    }

    fn print_with_operators(&self, operators: &[&char]) {
        print!("{}: ", self.test_value);

        for (i, operand) in self.operands.iter().enumerate() {
            print!("{}", operand);

            if i < operators.len() {
                print!(" {} ", operators[i]);
            }
        }

        println!(" \u{2713}");
    }

    fn is_valid_with_operators(&self, operators: &[&char]) -> Result<bool, String> {
        // Operators are evaluated from left to right not in normal order of operations
        if operators.is_empty() {
            return Ok(self.test_value == self.operands[0]);
        }

        let lhs = self.operands[0];
        let rhs = self.operands[1];
        let operator = operators[0];

        let result = match *operator {
            Self::OPERATOR_ADD => lhs + rhs,
            Self::OPERATOR_MULTIPLY => lhs * rhs,
            _ => return Err("Invalid operator".into()),
        };

        let new_operators = &operators[1..];
        let mut new_operands = self.operands[2..].to_vec();

        new_operands.insert(0, result);

        let new_equation = Equation {
            test_value: self.test_value,
            operands: new_operands.to_vec(),
        };

        new_equation.is_valid_with_operators(new_operators)
    }
}

impl From<&str> for Equation {
    fn from(s: &str) -> Self {
        let parts = s.split(":").collect::<Vec<&str>>();
        let test_value = parts[0].trim().parse::<i64>().unwrap();

        let operands = parts[1]
            .trim()
            .split(" ")
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        Equation {
            test_value,
            operands,
        }
    }
}

fn main() {
    let file_path = "./d7-bridge-repair/input.txt";
    let contents = fs::read_to_string(file_path).unwrap();

    let solution = contents
        .lines()
        .map(Equation::from)
        .filter(|e| e.is_valid().unwrap())
        .map(|e| e.test_value)
        .sum::<i64>();

    println!("Solution: {}", solution);
}
