use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    let records = parse(input);
    records
        .iter()
        .map(|nums| if is_valid(nums) { 1 } else { 0 })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    let records = parse(input);
    records
        .iter()
        .map(|nums| {
            if is_valid(nums) {
                return 1;
            }

            for to_drop in 0..nums.len() {
                let mut nums_copy = nums.clone();
                nums_copy.remove(to_drop);

                if is_valid(&nums_copy) {
                    return 1;
                }
            }
            0
        })
        .sum()
}

pub fn parse(input: &str) -> Vec<Vec<i8>> {
    input
        .lines()
        .map(|line| line.split(' ').map(|c| c.parse().unwrap()).collect())
        .collect()
}

pub fn is_valid(nums: &[i8]) -> bool {
    let asc = nums[0] < nums[1];

    nums.iter()
        .zip(nums.iter().skip(1))
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

    #[test]
    fn part2_sample() {
        let input = read_input("input/2024/sample/day2.txt");
        assert_eq!(part2(&input), 4);
    }
}
