use std::collections::VecDeque;

everybody_codes::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut mentors = 0;
    Some(
        input
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|c| {
                match c {
                    'A' => mentors += 1,
                    'a' => return mentors,
                    _ => {}
                }
                0
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut a_mentors = 0;
    let mut b_mentors = 0;
    let mut c_mentors = 0;
    Some(
        input
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|c| {
                match c {
                    'A' => a_mentors += 1,
                    'a' => return a_mentors,
                    'B' => b_mentors += 1,
                    'b' => return b_mentors,
                    'C' => c_mentors += 1,
                    'c' => return c_mentors,
                    _ => {}
                }
                0
            })
            .sum(),
    )
}

pub fn part_three(input: &str) -> Option<u64> {
    let mut input_vec = input.lines().next().unwrap().chars().collect::<Vec<char>>();
    let mut a_mentor_behind: VecDeque<usize> = VecDeque::new();
    let mut a_mentor_ahead: VecDeque<usize> = VecDeque::new();
    let mut b_mentor_behind: VecDeque<usize> = VecDeque::new();
    let mut b_mentor_ahead: VecDeque<usize> = VecDeque::new();
    let mut c_mentor_behind: VecDeque<usize> = VecDeque::new();
    let mut c_mentor_ahead: VecDeque<usize> = VecDeque::new();

    let mut i = 0;
    let mut frontier = 0;

    let mut middle_accum = 0;
    let mut ends_accum = 0;

    let input_len = input_vec.len();

    let mut temp_clone = input_vec.clone();

    input_vec.append(&mut temp_clone.clone());
    input_vec.append(&mut temp_clone);

    while frontier < 1000 {
        match input_vec[frontier % input_vec.len()] {
            'A' => a_mentor_ahead.push_back(frontier),
            'B' => b_mentor_ahead.push_back(frontier),
            'C' => c_mentor_ahead.push_back(frontier),
            _ => {}
        }
        frontier += 1;
    }

    while i < input_vec.len() {
        if frontier < input_vec.len() {
            match input_vec[frontier] {
                'A' => a_mentor_ahead.push_back(frontier),
                'B' => b_mentor_ahead.push_back(frontier),
                'C' => c_mentor_ahead.push_back(frontier),
                _ => {}
            }
        }
        let value = match input_vec[i] {
            'a' => shrink_stack(&mut a_mentor_behind, i) + a_mentor_ahead.len(),
            'b' => shrink_stack(&mut b_mentor_behind, i) + b_mentor_ahead.len(),
            'c' => shrink_stack(&mut c_mentor_behind, i) + c_mentor_ahead.len(),
            'A' => {
                a_mentor_behind.push_back(i);
                a_mentor_ahead.pop_front();
                0
            }
            'B' => {
                b_mentor_behind.push_back(i);
                b_mentor_ahead.pop_front();
                0
            }
            'C' => {
                c_mentor_behind.push_back(i);
                c_mentor_ahead.pop_front();
                0
            }
            _ => 0,
        };
        if i < input_len || i >= input_len * 2 {
            ends_accum += value;
        } else {
            middle_accum += value;
        }
        i += 1;
        frontier += 1;
    }

    Some(((middle_accum * 998) + ends_accum) as u64)
}

fn shrink_stack(stack: &mut VecDeque<usize>, pos: usize) -> usize {
    if stack.is_empty() {
        return 0;
    }
    while let Some(&front) = stack.front()
        && pos - front > 1000
    {
        stack.pop_front();
    }
    stack.len()
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
