everybody_codes::solution!(1);

use std::cmp::{max, min};

pub fn part_one(input: &str) -> Option<String> {
    let mut lines_iter = input.lines();
    let words = lines_iter.next().unwrap().split(',').collect::<Vec<&str>>();
    lines_iter.next().unwrap(); // Skip empty line
    let index = lines_iter.next().unwrap().split(',').fold(0, |acc, cmd| {
        let (turn, dist) = cmd.split_at(1);
        let dist: i32 = dist.parse().unwrap();
        match turn {
            "L" => max(0, acc - dist),
            "R" => min(acc + dist, words.len() as i32 - 1),
            _ => panic!("Invalid turn command"),
        }
    }) as usize;
    Some(words[index].to_owned())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut lines_iter = input.lines();
    let words = lines_iter.next().unwrap().split(',').collect::<Vec<&str>>();
    lines_iter.next().unwrap(); // Skip empty line
    let index = lines_iter.next().unwrap().split(',').fold(0, |acc, cmd| {
        let (turn, dist) = cmd.split_at(1);
        let dist: i32 = dist.parse().unwrap();
        let mut partial_index = match turn {
            "L" => acc - dist,
            "R" => acc + dist,
            _ => panic!("Invalid turn command"),
        };
        if partial_index >= words.len() as i32 {
            return partial_index % words.len() as i32;
        }
        while partial_index < 0 {
            partial_index += words.len() as i32;
        }
        partial_index
    }) as usize;
    Some(words[index].to_owned())
}

pub fn part_three(input: &str) -> Option<String> {
    let mut lines_iter = input.lines();
    let mut words = lines_iter.next().unwrap().split(',').collect::<Vec<&str>>();
    lines_iter.next().unwrap(); // Skip empty line
    lines_iter.next().unwrap().split(',').for_each(|cmd| {
        let (turn, dist) = cmd.split_at(1);
        let dist: usize = dist.parse().unwrap();
        let mut partial_index = match turn {
            "L" => 0 - (dist as i32),
            "R" => dist as i32,
            _ => panic!("Invalid turn command"),
        };
        if partial_index >= words.len() as i32 {
            partial_index %= words.len() as i32;
        }
        while partial_index < 0 {
            partial_index += words.len() as i32;
        }
        words.swap(0, partial_index as usize);
    });
    Some(words[0].to_owned())
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
