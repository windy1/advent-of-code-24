use std::fs;

fn calculate_nth_secret_number(start: usize, n: usize) -> usize {
    let mut secret = start;

    for _ in 0..n {
        secret = generate_next_secret_number(secret);
    }

    secret
}

fn generate_next_secret_number(secret: usize) -> usize {
    let mut salt = secret * 64;
    let mut secret = prune_secret(mix_secret(secret, salt));
    salt = secret / 32;
    secret = prune_secret(mix_secret(secret, salt));
    salt = secret * 2048;
    prune_secret(mix_secret(secret, salt))
}

fn mix_secret(secret: usize, salt: usize) -> usize {
    secret ^ salt
}

fn prune_secret(secret: usize) -> usize {
    secret % 16777216
}

fn main() {
    let file_path = "./d22-monkey-market/input.txt";
    let contents = fs::read_to_string(file_path).unwrap().trim().to_string();

    let result = contents
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .map(|x| calculate_nth_secret_number(x, 2000))
        .sum::<usize>();

    println!("Solution (Part 1): {}", result);
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn test_mix_secret() {
        let secret = 42;
        let salt = 15;
        let result = mix_secret(secret, salt);
        let expected = 37;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_prune_secret() {
        let secret = 100000000;
        let result = prune_secret(secret);
        let expected = 16113920;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_next_secret_number() {
        let secret = 123;
        let result = generate_next_secret_number(secret);
        let expected = 15887950;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_calculate_nth_secret_number() {
        let secret = 1;
        let n = 2000;
        let result = calculate_nth_secret_number(secret, n);
        let expected = 8685429;
        assert_eq!(result, expected);
    }
}
