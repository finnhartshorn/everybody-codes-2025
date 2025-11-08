everybody_codes::solution!(3);

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input.lines()
            .next()
            .unwrap()
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect::<HashSet<u32>>()
            .into_iter()
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut bt_set = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect::<BTreeSet<u32>>();
    let mut result: u32 = 0;
    for _ in 0..20 {
        result += bt_set.pop_first().unwrap();
    }
    Some(result)
}

pub fn part_three(input: &str) -> Option<u32> {
    let mut map = BTreeMap::new();
    input.lines().next().unwrap().split(',').for_each(|c| {
        let p: u32 = c.parse().unwrap();
        map.entry(p).and_modify(|curr| *curr += 1).or_insert(1);
    });

    let mut sets = 0;
    let mut empty = false;

    while !empty {
        empty = true;
        map.iter_mut().for_each(|(key, value)| {
            if *value != 0 {
                *value -= 1;
                empty = false;
            }
        });
        if !empty {
            sets += 1
        }
    }

    Some(sets)
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
