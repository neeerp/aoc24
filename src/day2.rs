use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    input.lines().map(parse_line).sum()
}

pub fn parse_line(line: &str) -> i32 {
    let parts = line.split(' ').map(|c| c.parse().unwrap()).collect();
    if is_valid(parts) {
        1
    } else {
        0
    }
}

pub fn is_valid(parts: Vec<i32>) -> bool {
    let asc = parts[0] < parts[1];

    parts
        .iter()
        .zip(parts.iter().skip(1))
        .fold(true, |valid, (left, right)| {
            let mono_step = left < right && asc || (left > right && !asc);
            let dist = (left - right).abs();
            let small_step = (1..=3).contains(&dist);

            valid && mono_step && small_step
        })
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
        let input = read_input("input/2024/sample/day2.txt");
        assert_eq!(part1(&input), 2);
    }
}
