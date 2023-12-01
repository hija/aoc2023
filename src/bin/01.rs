advent_of_code::solution!(1);

pub fn get_numbers(input: &str) -> u32 {
    let mut numbers = input.chars().filter_map(|a| a.to_digit(10));
    // println!("{:?}", numbers);
    let first = numbers.next().unwrap();
    let last = numbers.last();
    // println!("{} {}", first, last.unwrap_or(first));
    return first * 10 + last.unwrap_or(first);
}

pub fn preprocess_line(input: &str) -> String {
    let list_of_replacements = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    let mut complete_string = "".to_string();
    let mut prev_string = "".to_string();
    for ch in input.chars() {
        if ch.is_digit(10) {
            complete_string.push(ch);
        } else {
            prev_string.push(ch);

            for replacement in list_of_replacements {
                if prev_string.contains(replacement.0) {
                    complete_string.push(replacement.1);
                    prev_string.clear();
                    prev_string.push(replacement.0.chars().last().unwrap());
                }
            }
        }
    }

    return complete_string;
}

pub fn part_one(input: &str) -> Option<u32> {
    return input.lines().map(get_numbers).reduce(|acc, e| acc + e);
}

pub fn part_two(input: &str) -> Option<u32> {
    return input
        .lines()
        .map(preprocess_line)
        .map(|a| get_numbers(&a))
        .reduce(|acc, e| acc + e);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
