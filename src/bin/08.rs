everybody_codes::solution!(8);

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u64> {
    let size = if input.len() < 20 { 4 } else { 16 };

    Some(
        input
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse::<i64>().unwrap())
            .tuple_windows()
            .filter(|(a, b)| (b - a).abs() == size)
            .count() as u64,
    )
}

fn check_if_cross(new: (i64, i64), existing: (i64, i64)) -> bool {
    let existing_range = if existing.0 < existing.1 {
        existing.0..=existing.1
    } else {
        existing.1..=existing.0
    };
    existing_range.contains(&new.0) != existing_range.contains(&new.1)
        && new.0 != existing.0
        && new.0 != existing.1
        && new.1 != existing.0
        && new.1 != existing.1
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut existing_connections: HashSet<(i64, i64)> = HashSet::new();
    Some(
        input
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse::<i64>().unwrap())
            .tuple_windows()
            .map(|new_connection: (i64, i64)| {
                let knots = existing_connections
                    .iter()
                    .filter(|existing_connection| {
                        check_if_cross(new_connection, **existing_connection)
                    })
                    .count() as u64;
                existing_connections.insert(new_connection);
                knots
            })
            .sum(),
    )
}

pub fn part_three(input: &str) -> Option<u64> {
    let size = if input.len() < 20 { 8 } else { 256 };
    let existing_connections: HashMap<(_, _), _> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .tuple_windows()
        .counts();

    let mut max_cuts = 0;

    for i in 1..=size {
        for j in 1..=size {
            let mut cuts = existing_connections
                .iter()
                .map(|existing_connection| {
                    if check_if_cross((i, j), *existing_connection.0) {
                        *existing_connection.1
                    } else {
                        0
                    }
                })
                .sum::<usize>() as u64;
            if let Some(v) = existing_connections.get(&(i, j)) {
                cuts += *v as u64
            }
            if let Some(v) = existing_connections.get(&(j, i)) {
                cuts += *v as u64
            }
            if cuts > max_cuts {
                max_cuts = cuts;
            }
        }
    }

    Some(max_cuts)
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
