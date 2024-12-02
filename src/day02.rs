pub fn valid_report(report: &str) -> i32 {
    let report: Vec<i32> = report
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
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
        let reports: Vec<i32> = TEST_INPUT
            .lines()
            .map(|report| valid_report(report))
            .collect();
        assert_eq!(reports, vec![1, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn count_valid_reports_in_puzzle_input() {
        let input = fs::read_to_string("puzzle_input/02.txt").unwrap();
        assert_eq!(input.lines().map(|report| valid_report(report)).sum::<i32>(), 202i32);
    }
}
