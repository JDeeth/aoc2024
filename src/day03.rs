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

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const TEST_INPUT: &str = "\
xmul(2,4)%&mul[3,7]!@^do_
not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn sum_valid_mul_in_test() {
        assert_eq!(parse_mul(TEST_INPUT), 161);
    }

    #[test]
    fn sum_valid_mul_in_puzzle() {
        let input = fs::read_to_string("puzzle_input/03.txt").unwrap();
        assert_eq!(parse_mul(&input), 173419328);
    }
}
