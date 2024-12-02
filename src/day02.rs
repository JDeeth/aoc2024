fn parse_report(report: &str) -> Vec<i32> {
    report
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn valid_report(report: Vec<i32>) -> i32 {
    let intervals: Vec<_> = report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(&a, &b)| b - a)
        .collect();
    match () {
        _ if intervals.iter().all(|x| (1..=3).contains(x)) => 1,
        _ if intervals.iter().all(|x| (-3..=-1).contains(x)) => 1,
        _ => 0,
    }
}

pub fn validate(reports: &str) -> Vec<i32> {
    reports
        .lines()
        .map(parse_report)
        .map(valid_report)
        .collect()
}

fn valid_with_tolerance(report: Vec<i32>) -> i32 {
    for n in 0..report.len() {
        if valid_report(
            report
                .iter()
                .enumerate()
                .filter(|&(i, _)| i != n)
                .map(|(_, &v)| v)
                .collect(),
        ) == 1
        {
            return 1;
        }
    }
    0
}

pub fn validate_with_tolerance(reports: &str) -> Vec<i32> {
    reports
        .lines()
        .map(parse_report)
        .map(valid_with_tolerance)
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    const TEST_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn valid_report_on_test_input() {
        assert_eq!(validate(TEST_INPUT), vec![1, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn count_valid_reports_in_puzzle_input() {
        let input = fs::read_to_string("puzzle_input/02.txt").unwrap();
        assert_eq!(validate(&input).iter().sum::<i32>(), 202);
    }

    #[test]
    fn valid_report_on_test_input_with_tolerance() {
        assert_eq!(validate_with_tolerance(TEST_INPUT), vec![1, 0, 0, 1, 1, 1]);
    }

    #[test]
    fn count_valid_reports_in_puzzle_input_with_tolerance() {
        let input = fs::read_to_string("puzzle_input/02.txt").unwrap();
        assert_eq!(validate_with_tolerance(&input).iter().sum::<i32>(), 271);
    }
}
