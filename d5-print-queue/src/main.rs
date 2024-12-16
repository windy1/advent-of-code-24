use std::fs;

type PrintRule = [i32; 2];

struct PrintRules {
    rules: Vec<PrintRule>,
}

impl PrintRules {
    fn verify_update(&self, update: Vec<i32>) -> bool {
        for current_page_index in 0..update.len() {
            let page = update[current_page_index];
            let previous_page_rules = self.previous_page_rules(page);
            let next_page_rules = self.next_page_rules(page);

            for &previous_page in &update[..current_page_index] {
                let should_be_before_current_page = previous_page_rules
                    .iter()
                    .any(|rule| rule[0] == previous_page);

                if !should_be_before_current_page {
                    return false;
                }
            }

            for &next_page in &update[(current_page_index + 1)..] {
                let should_be_after_current_page =
                    next_page_rules.iter().any(|rule| rule[1] == next_page);

                if !should_be_after_current_page {
                    return false;
                }
            }
        }

        true
    }

    fn previous_page_rules(&self, page: i32) -> Vec<&PrintRule> {
        self.rules.iter().filter(|rule| rule[1] == page).collect()
    }

    fn next_page_rules(&self, page: i32) -> Vec<&PrintRule> {
        self.rules.iter().filter(|rule| rule[0] == page).collect()
    }
}

impl From<&str> for PrintRules {
    fn from(rules_str: &str) -> Self {
        let rules = rules_str
            .split("\n")
            .map(|rule_str| {
                let rule_parts = rule_str.split("|").collect::<Vec<&str>>();

                [
                    rule_parts[0].parse::<i32>().unwrap(),
                    rule_parts[1].parse::<i32>().unwrap(),
                ]
            })
            .collect();

        PrintRules { rules }
    }
}

struct PrintUpdates {
    updates: Vec<Vec<i32>>,
}

impl From<&str> for PrintUpdates {
    fn from(updates_str: &str) -> Self {
        let updates = updates_str
            .split("\n")
            .map(|update_str| {
                update_str
                    .split(",")
                    .map(|num_str| num_str.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();

        PrintUpdates { updates }
    }
}

fn main() {
    let file_path = "./d5-print-queue/input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let input_parts = contents.split("\n\n").collect::<Vec<&str>>();
    let print_rules_str = input_parts[0].trim();
    let updates_str = input_parts[1].trim();

    let print_rules = PrintRules::from(print_rules_str);
    let print_updates = PrintUpdates::from(updates_str);
    let mut acc = 0;

    for update in &print_updates.updates {
        let is_valid = print_rules.verify_update(update.to_vec());

        if is_valid {
            acc += update[update.len() / 2];
        }
    }

    println!("Solution: {}", acc);
}
