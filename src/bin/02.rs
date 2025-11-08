everybody_codes::solution!(2);

use std::collections::HashSet;

fn add(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0 + b.0, a.1 + b.1)
}

fn multiply(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    // X1 * X2 - Y1 * Y2, X1 * Y2 + Y1 * X2
    (a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0)
}

fn divide(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (a.0 / b.0, a.1 / b.1)
}

pub fn part_one(input: &str) -> Option<String> {
    let val_str = input
        .lines()
        .next()
        .unwrap()
        .trim_start_matches("A=[")
        .trim_end_matches(']');
    let a = tuple_parse(val_str);
    let mut result = (0, 0);
    for _ in 0..3 {
        result = multiply(result, result);
        result = divide(result, (10, 10));
        result = add(result, a);
    }

    Some(format!("[{},{}]", result.0, result.1).to_string())
}

fn tuple_parse(s: &str) -> (i64, i64) {
    let nums: Vec<i64> = s.split(',').map(|x| x.trim().parse().unwrap()).collect();
    (nums[0], nums[1])
}

pub fn part_two(input: &str) -> Option<u64> {
    let val_str = input
        .lines()
        .next()
        .unwrap()
        .trim_start_matches("A=[")
        .trim_end_matches(']');
    let a = tuple_parse(val_str);
    let mut unique_points: HashSet<(i64, i64)> = HashSet::new();

    for i in (a.0..=a.0 + 1000).step_by(10) {
        for j in (a.1..=a.1 + 1000).step_by(10) {
            if should_engrave(i, j) {
                unique_points.insert((i, j));
            }
        }
    }

    Some(unique_points.len() as u64)
}

fn should_engrave(x: i64, y: i64) -> bool {
    let mut result = (0, 0);
    for _ in 0..100 {
        result = multiply(result, result);
        result = divide(result, (100_000, 100_000));
        result = add(result, (x, y));
        if result.0.abs() > 1000_000 || result.1.abs() > 1000_000 {
            return false;
        }
    }
    true
}

pub fn part_three(input: &str) -> Option<u64> {
    let val_str = input
        .lines()
        .next()
        .unwrap()
        .trim_start_matches("A=[")
        .trim_end_matches(']');
    let a = tuple_parse(val_str);
    let mut unique_points: HashSet<(i64, i64)> = HashSet::new();

    for i in a.0..=a.0 + 1000 {
        for j in a.1..=a.1 + 1000 {
            if should_engrave(i, j) {
                unique_points.insert((i, j));
            }
        }
    }

    Some(unique_points.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&everybody_codes::template::read_file("samples", DAY, 1));
        let expected = everybody_codes::template::read_file("answers", DAY, 1)
            .trim()
            .parse()
            .ok();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&everybody_codes::template::read_file("samples", DAY, 2));
        let expected = everybody_codes::template::read_file("answers", DAY, 2)
            .trim()
            .parse()
            .ok();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_three() {
        let result = part_three(&everybody_codes::template::read_file("samples", DAY, 3));
        let expected = everybody_codes::template::read_file("answers", DAY, 3)
            .trim()
            .parse()
            .ok();
        assert_eq!(result, expected);
    }
}
