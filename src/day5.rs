use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

type RuleMap<'a> = HashMap<&'a str, Vec<&'a str>>;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> u32 {
    let (rule_map, order_list) = parse(input);

    order_list.iter().map(|ordering| {
        if check(ordering, &rule_map) {
            ordering[(ordering.len()) / 2].parse().unwrap()
        } else {
            0
        }
    }).sum()
}

pub fn check(ordering: &Vec<&str>, rule_map: &RuleMap) -> bool {
    let mut seen: HashSet<&str> = HashSet::new();

    for cur in ordering.iter().rev() {
        if !rule_map.contains_key(cur) {
            seen.insert(cur);
            continue
        }

        for dep in &rule_map[cur] {
            if seen.contains(dep) {
                return false;
            }
        }

        seen.insert(cur);
    }

    true
}


pub fn parse(input: &str) -> (RuleMap, Vec<Vec<&str>>) {
    let (rules, orderings) = input.split_once("\n\n").unwrap();

    let mut rule_map: HashMap<&str, Vec<&str>> = HashMap::new();
    rules.lines().for_each(|line| {
        let (v, k) = line.split_once('|').unwrap();
        rule_map.entry(k).or_default().push(v);
    });

    let order_list = orderings.lines().map(|line| {
        line.split(',').collect()
    }).collect();


    (rule_map, order_list)
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
        let input = read_input("input/2024/sample/day5.txt");
        assert_eq!(part1(&input), 143);
    }
}
