everybody_codes::solution!(7);

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<String> {
    let mut input_iter = input.lines();
    let mut words = input_iter.next().unwrap().split(',');
    input_iter.next().unwrap(); // Skip empty line

    let rules: HashMap<char, HashSet<char>> = input_iter
        .map(|l| {
            let mut line_iter = l.split(" > ");
            let starting_letter: char = line_iter.next().unwrap().parse().unwrap();
            let ending_letters: HashSet<char> = line_iter
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            (starting_letter, ending_letters)
        })
        .collect();

    Some(
        words
            .find(|w| {
                w.chars()
                    .tuple_windows()
                    .all(|(a, b)| rules.get(&a).unwrap().contains(&b))
            })
            .unwrap_or_default()
            .to_owned(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input_iter = input.lines();
    let words = input_iter.next().unwrap().split(',');
    input_iter.next().unwrap(); // Skip empty line

    let rules: HashMap<char, HashSet<char>> = input_iter
        .map(|l| {
            let mut line_iter = l.split(" > ");
            let starting_letter: char = line_iter.next().unwrap().parse().unwrap();
            let ending_letters: HashSet<char> = line_iter
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            (starting_letter, ending_letters)
        })
        .collect();

    Some(words.enumerate().fold(0, |acc, (i, w)| {
        if w.chars()
            .tuple_windows()
            .all(|(a, b)| rules.get(&a).unwrap().contains(&b))
        {
            acc + i + 1
        } else {
            acc
        }
    }) as u64)
}

pub fn part_three(input: &str) -> Option<u64> {
    let mut input_iter = input.lines();
    let words = input_iter.next().unwrap().split(',');
    input_iter.next().unwrap(); // Skip empty line

    let rules: HashMap<char, HashSet<char>> = input_iter
        .map(|l| {
            let mut line_iter = l.split(" > ");
            let starting_letter: char = line_iter.next().unwrap().parse().unwrap();
            let ending_letters: HashSet<char> = line_iter
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            (starting_letter, ending_letters)
        })
        .collect();

    let filtered_words: HashSet<&str> = words
        .filter(|w| {
            w.chars()
                .tuple_windows()
                .all(|(a, b)| rules.get(&a).unwrap().contains(&b))
        })
        .collect();

    let filtered_words: Vec<&str> = filtered_words
        .clone()
        .into_iter()
        .filter(|w| {
            !filtered_words
                .clone()
                .into_iter()
                .any(|p| w.starts_with(p) && *w != p)
        })
        .collect();

    Some(
        filtered_words
            .iter()
            .map(|w| {
                let mut memo_map: HashMap<(char, u64), u64> = HashMap::new();
                (7..=11)
                    .map(|word_length| {
                        if w.len() > word_length {
                            0
                        } else if w.len() == word_length {
                            1
                        } else {
                            let m = memo(
                                &mut memo_map,
                                &rules,
                                w.as_bytes()[w.len() - 1] as char,
                                (word_length - w.len()) as u64,
                            );
                            m
                        }
                    })
                    .sum::<u64>()
            })
            .sum(),
    )
}

fn memo(
    memo_map: &mut HashMap<(char, u64), u64>,
    rules: &HashMap<char, HashSet<char>>,
    c: char,
    level: u64,
) -> u64 {
    if level == 0 {
        return 1;
    }

    if !rules.contains_key(&c) {
        return 0;
    }

    if let Some(entry) = memo_map.get(&(c, level)) {
        return *entry;
    }

    let new_entry = rules
        .get(&c)
        .unwrap()
        .iter()
        .map(|next_char| memo(memo_map, rules, *next_char, level - 1))
        .sum();

    memo_map.insert((c, level), new_entry);

    new_entry
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
