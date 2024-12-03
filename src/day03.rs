use regex::Regex;

pub fn parse_mul(text: &str) -> u64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result = 0u64;
    for (_, [a, b]) in re.captures_iter(text).map(|c| c.extract()) {
        let a = a.parse::<u64>().unwrap();
        let b = b.parse::<u64>().unwrap();
        result += a * b;
    }
    result
}

pub fn parse_conditionals(text: &str) -> u64 {
    text.split("do()")
        .map(|x| x.split_once("don't()").unwrap_or((x, "")).0)
        .map(parse_mul)
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const TEST_INPUT_1: &str = "\
xmul(2,4)%&mul[3,7]!@^do_
not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const TEST_INPUT_2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn sum_valid_mul_in_test() {
        assert_eq!(parse_mul(TEST_INPUT_1), 161);
    }

    #[test]
    fn sum_valid_mul_in_puzzle() {
        let input = fs::read_to_string("puzzle_input/03.txt").unwrap();
        assert_eq!(parse_mul(&input), 173419328);
    }

    #[test]
    fn test_with_conditionals() {
        assert_eq!(parse_conditionals(TEST_INPUT_2), 48);
    }

    #[test]
    fn sum_valid_mul_in_puzzle_with_conditionals() {
        let input = fs::read_to_string("puzzle_input/03.txt").unwrap();
        assert_eq!(parse_conditionals(&input), 90669332);
    }
}
