use std::{collections::HashMap, fs, iter::zip};

pub fn run_day1() {
    let day1_input = read_input("inputs/day1_input.txt");
    let day1p1_result = day1p1(&day1_input);
    println!("The result for day 1 part 1 is: {}", day1p1_result);
    let day1p2_result = day1p2(&day1_input);
    println!("The result for day 1 part 2 is: {}", day1p2_result);
}

fn day1p1(input: &str) -> isize {
    let (mut first, mut second) = parse_d1(input);

    first.sort();
    second.sort();

    let sum: isize = zip(first, second).map(|(a, b)| (a - b).abs()).sum();
    sum
}

fn day1p2(input: &str) -> isize {
    let (first, second) = parse_d1(input);

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

fn parse_d1(input: &str) -> (Vec<isize>, Vec<isize>) {
    input
        .split('\n')
        .fold((vec![], vec![]), |(mut first, mut second), line| {
            let parts: Vec<&str> = line.split("   ").collect();
            match parts[..] {
                [a, b] => {
                    first.push(a.parse().unwrap());
                    second.push(b.parse().unwrap());
                    (first, second)
                }
                _ => panic!("Oops!"),
            }
        })
}

fn read_input(file_name: &str) -> String {
    fs::read_to_string(file_name)
        .expect("Unable to read input!")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = read_input("inputs/day1_sample.txt");
        assert_eq!(day1p1(&input), 11);
    }

    #[test]
    fn part1_actual() {
        let input = read_input("inputs/day1_input.txt");
        assert_eq!(day1p1(&input), 2756096);
    }

    #[test]
    fn part2_sample() {
        let input = read_input("inputs/day1_sample.txt");
        assert_eq!(day1p2(&input), 31);
    }

    #[test]
    fn part2_actual() {
        let input = read_input("inputs/day1_input.txt");
        assert_eq!(day1p2(&input), 23117829);
    }
}
