advent_of_code::solution!(1);
// use aho_corasick::AhoCorasick;

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(get_numbers).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(replace_text_with_digits)
            .map(|i| get_numbers(&i))
            .sum(),
        // find_numbers_with_aho_corasick(input),
    )
}

fn get_numbers(line: &str) -> u32 {
    let first = line.chars().find_map(|n| n.to_digit(10)).unwrap();
    let last = line.chars().rev().find_map(|n| n.to_digit(10)).unwrap();
    10 * first + last
}

fn replace_text_with_digits(text: &str) -> String {
    let nums = vec![
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];
    let mut text = text.to_string();
    for (key, value) in nums {
        text = text.replace(key, value);
    }

    text
}

// fn find_numbers_with_aho_corasick(lines: &str) -> u32 {
//     let patterns = vec![
//         "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
//         "eight", "8", "nine", "9",
//     ];
//     let ac = AhoCorasick::new(patterns).unwrap();
//     let mut total = 0;
//     for line in lines.lines() {
//         let matches = ac.find_overlapping_iter(line).collect::<Vec<_>>();
//         let first = matches.get(0).unwrap().pattern().as_usize() / 2 + 1;
//         let last = matches.iter().last().unwrap().pattern().as_usize() / 2 + 1;
//         total += 10 * (first) + (last)
//     }
//     total as u32
// }

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
