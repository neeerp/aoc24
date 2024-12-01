use std::{collections::HashMap, iter::zip};

use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let (mut first, mut second) = parse(input);

    first.sort();
    second.sort();

    let sum: i32 = zip(first, second).map(|(a, b)| (a - b).abs()).sum();
    sum
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let (first, second) = parse(input);

    let counts = second.iter().fold(HashMap::new(), |mut acc, val| {
        let count = acc.entry(val).or_insert(0);
        *count += 1;

        acc
    });

    first.iter().fold(0, |total, val| {
        let occ = *counts.get(val).unwrap_or(&0);
        total + val * occ
    })
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    input.split('\n').fold(
        (Vec::with_capacity(1000), Vec::with_capacity(1000)),
        |(mut first, mut second), line| {
            let parts = line.split_once("   ");
            if let Some((a, b)) = parts {
                first.push(a.parse().unwrap());
                second.push(b.parse().unwrap());
            }
            (first, second)
        },
    )
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
        let input = read_input("input/2024/sample/day1.txt");
        assert_eq!(part1(&input), 11);
    }

    #[test]
    fn part2_sample() {
        let input = read_input("input/2024/sample/day1.txt");
        assert_eq!(part2(&input), 31);
    }
}
