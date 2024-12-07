use std::collections::HashSet;

use aoc_runner_derive::aoc;

const START_SYMBOL: char = '^';
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let grid = input.split('\n').collect::<Vec<&str>>();

    let m = grid.len() as i64;
    let n = grid[0].len() as i64;
    let start = find_start(&grid).unwrap();

    // Brute force: Simulation
    let (mut i, mut j) = start;
    let mut dir = Direction::Up;
    let mut seen: HashSet<(i64, i64)> = HashSet::new();
    loop {
        seen.insert((i, j));

        let (next_i, next_j) = get_next(i, j, &dir);
        if !is_in_bounds(next_i, next_j, m, n) {
            break;
        }

        if let Some('#') = grid[next_i as usize].chars().nth(next_j as usize) {
            dir = match dir {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
        } else {
            i = next_i;
            j = next_j;
        }
    }

    seen.len()
}

fn is_in_bounds(i: i64, j: i64, m: i64, n: i64) -> bool {
    i >= 0 && j >= 0 && i < m && j < n
}

fn get_next(i: i64, j: i64, dir: &Direction) -> (i64, i64) {
    match dir {
        Direction::Up => (i - 1, j),
        Direction::Right => (i, j + 1),
        Direction::Down => (i + 1, j),
        Direction::Left => (i, j - 1),
    }
}

fn find_start(grid: &[&str]) -> Option<(i64, i64)> {
    for (i, line) in grid.iter().enumerate() {
        if let Some(j) = line.find(START_SYMBOL) {
            return Some((i as i64, j as i64));
        }
    }

    None
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
        let input = read_input("input/2024/sample/day6.txt");
        assert_eq!(part1(&input), 41);
    }
}
