advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    dbg!(input);
    let mut result = 0;
    for part in input.split("\n") {
        let mut found_digit_times = 0;
        let mut first = 0;
        let mut last = 0;
        for ch in part.chars() {
            if (ch.is_digit(10)) {
                if found_digit_times == 0 {
                    first = ch.to_digit(10).unwrap();
                    last = ch.to_digit(10).unwrap();
                } else {
                    last = ch.to_digit(10).unwrap();
                }
                found_digit_times += 1;
            }
            // if (ch.is_digit(10) && found_digit_times == 0) {
            // first = ch.to_digit(10).unwrap();
            // found_digit_times += 1;
            // } else {
            // found_digit_times += 1;
            // match ch {}
            // last = ch.to_digit(10).unwrap();
            // }
            // dbg!(ch);
            // dbg!(ch.is_digit(10));
            // dbg!(found_digit_times);
        }
        result += first * 10 + last;
        dbg!(first, last, result);
        dbg!(part);
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 142);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
