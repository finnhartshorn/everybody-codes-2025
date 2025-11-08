everybody_codes::solution!(5);

use itertools::Itertools;
use std::cmp::Ordering;

struct Segment {
    spine: u64,
    left: Option<u64>,
    right: Option<u64>,
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(calc_quality(input.lines().next().unwrap()).1)
}

fn calc_quality(line: &str) -> (Vec<Segment>, u64, u64) {
    let mut sword: Vec<Segment> = Vec::new();
    let mut answer: String = "".to_string();
    let mut line_iter = line.split(':');
    let id = line_iter.next().unwrap().parse().unwrap();

    let mut char_iter = line_iter
        .next()
        .unwrap()
        .split(',')
        .map(|c| c.parse::<u64>().unwrap());
    sword.push(Segment {
        spine: char_iter.next().unwrap(),
        left: None,
        right: None,
    });
    answer += &format!("{}", sword[0].spine);
    'outer: for n in char_iter {
        for segment in sword.iter_mut() {
            if n < segment.spine && segment.left.is_none() {
                segment.left = Some(n);
                continue 'outer;
            } else if n > segment.spine && segment.right.is_none() {
                segment.right = Some(n);
                continue 'outer;
            }
        }
        sword.push(Segment {
            spine: n,
            left: None,
            right: None,
        });
        answer += &format!("{}", n);
    }
    (sword, answer.parse().unwrap(), id)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut minimum: u64 = u64::MAX;
    let mut maximum: u64 = u64::MIN;
    input.lines().for_each(|line| {
        let quality = calc_quality(line).1;
        if quality < minimum {
            minimum = quality;
        }
        if quality > maximum {
            maximum = quality;
        }
    });
    Some(maximum - minimum)
}

pub fn part_three(input: &str) -> Option<u64> {
    let sorted_swords = input
        .lines()
        .map(|line| calc_quality(line))
        .sorted_unstable_by(|a, b| {
            if a.1 > b.1 {
                Ordering::Less
            } else if a.1 < b.1 {
                Ordering::Greater
            } else {
                for i in 0..a.0.len() {
                    let seg_a = seg_number(&a.0[i]);
                    let seg_b = seg_number(&b.0[i]);
                    if seg_a > seg_b {
                        return Ordering::Less;
                    } else if seg_a < seg_b {
                        return Ordering::Greater;
                    }
                }
                Ord::cmp(&b.2, &a.2)
            }
        });

    Some(
        sorted_swords
            .enumerate()
            .fold(0, |a, (i, sword)| a + ((i as u64 + 1) * sword.2)),
    )
}

fn seg_number(segment: &Segment) -> u64 {
    let mut answer: String = "".to_string();
    if let Some(left) = segment.left {
        answer += &format!("{}", left);
    }
    answer += &format!("{}", segment.spine);
    if let Some(right) = segment.right {
        answer += &format!("{}", right);
    }
    answer.parse().unwrap()
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
