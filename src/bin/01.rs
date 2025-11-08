everybody_codes::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

pub fn part_three(input: &str) -> Option<u64> {
    None
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
