everybody_codes::solution!(4);

use std::cmp::min;

pub fn part_one(input: &str) -> Option<u32> {
    let mut gears = input.lines().map(|l| l.parse::<u32>().unwrap());
    let first = gears.next().unwrap();
    let last = gears.next_back().unwrap();
    Some(first * 2025 / last)
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut gears = input.lines().map(|l| l.parse::<u128>().unwrap());
    let first = gears.next().unwrap();
    let last = gears.next_back().unwrap();
    let part = 10000000000000u128 * last;
    Some(part / first + min(1, part % first))
}

pub fn part_three(input: &str) -> Option<u64> {
    let mut lines_iter = input.lines();
    let first: u64 = lines_iter.next().unwrap().parse().unwrap();
    let last: u64 = lines_iter.next_back().unwrap().parse().unwrap();
    let ratio = lines_iter
        .map(|line| {
            let (a, b) = line.split_once('|').unwrap();
            b.parse::<u64>().unwrap() / a.parse::<u64>().unwrap()
        })
        .product::<u64>();

    Some(first * 100 * ratio / last)
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
