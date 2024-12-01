pub fn parse_col(input: &str, col: usize) -> Vec<u32> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .nth(col)
                .unwrap()
                .parse::<u32>()
                .unwrap()
        })
        .collect()
}

pub fn sum_distance(input: &str) -> u32 {
    let mut left = parse_col(input, 0);
    left.sort();
    let mut right = parse_col(input, 1);
    right.sort();
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| if a > b { a - b } else { b - a })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const TEST_INPUT: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn parse_2col_input() {
        assert_eq!(parse_col(TEST_INPUT, 0), vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(parse_col(TEST_INPUT, 1), vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn sum_distance_on_test_values() {
        assert_eq!(sum_distance(TEST_INPUT), 11);
    }

    #[test]
    fn sum_distance_on_puzzle_input() {
        let input = fs::read_to_string("src/input.txt").unwrap();
        assert_eq!(sum_distance(&input), 1151792);
    }
}
