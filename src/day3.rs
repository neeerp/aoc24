use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    re.captures_iter(input)
        .map(|c| c.extract())
        .fold(0, |sum, (_, [a, b])| {
            sum + a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()
        })
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    // TIL:
    // - The (.*?) group is a LAZY match; it causes the regex engine to find the SHORTEST match.
    // Otherwise, the greedy (.*) will find the LONGEST match that satisfies the rest of the
    // expression.
    //
    // - '.' does NOT match '\n's! To make it match '\n' as well, I added the (?s) flag at
    // the start of the regex.
    //
    // - Prefixing a group with '?:' makes it a non-capture group
    let segments_re = Regex::new(r"(?s)(?:^|do\(\))(.*?)(?:don't\(\)|$)").unwrap();

    segments_re
        .captures_iter(input)
        .map(|c| c.extract())
        .fold(0, |sum, (_, [segment])| sum + part1(segment))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn read_input(file_name: &str) -> String {
        fs::read_to_string(file_name)
            .expect("Unable to read input!")
            .trim()
            .to_string()
    }

    #[test]
    fn part1_sample() {
        let input = read_input("input/2024/sample/day3.txt");
        assert_eq!(part1(&input), 161);
    }

    #[test]
    fn part2_sample_on_p1() {
        let input = read_input("input/2024/sample/day3.txt");
        assert_eq!(part2(&input), 161);
    }

    #[test]
    fn part2_sample() {
        let input = read_input("input/2024/sample/day3p2.txt");
        assert_eq!(part2(&input), 48);
    }
}
