use aoc_runner_derive::aoc;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    (0..grid.len())
        .map(|i| (0..grid[0].len()).map(|j| find(i, j, &grid)).sum::<u32>())
        .sum()
}

pub fn find(i: usize, j: usize, grid: &[Vec<char>]) -> u32 {
    if grid[i][j] != 'X' {
        return 0;
    }

    let m = grid.len();
    let n = grid[0].len();
    let mut count = 0;

    if i > 2 && grid[i - 1][j] == 'M' && grid[i - 2][j] == 'A' && grid[i - 3][j] == 'S' {
        count += 1
    }

    if i > 2
        && j > 2
        && grid[i - 1][j - 1] == 'M'
        && grid[i - 2][j - 2] == 'A'
        && grid[i - 3][j - 3] == 'S'
    {
        count += 1
    }

    if i > 2
        && j < n - 3
        && grid[i - 1][j + 1] == 'M'
        && grid[i - 2][j + 2] == 'A'
        && grid[i - 3][j + 3] == 'S'
    {
        count += 1
    }

    if i < m - 3 && grid[i + 1][j] == 'M' && grid[i + 2][j] == 'A' && grid[i + 3][j] == 'S' {
        count += 1
    }

    if i < m - 3
        && j > 2
        && grid[i + 1][j - 1] == 'M'
        && grid[i + 2][j - 2] == 'A'
        && grid[i + 3][j - 3] == 'S'
    {
        count += 1
    }

    if i < m - 3
        && j < n - 3
        && grid[i + 1][j + 1] == 'M'
        && grid[i + 2][j + 2] == 'A'
        && grid[i + 3][j + 3] == 'S'
    {
        count += 1
    }

    if j > 2 && grid[i][j - 1] == 'M' && grid[i][j - 2] == 'A' && grid[i][j - 3] == 'S' {
        count += 1
    }

    if j < n - 3 && grid[i][j + 1] == 'M' && grid[i][j + 2] == 'A' && grid[i][j + 3] == 'S' {
        count += 1
    }

    count
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
        let input = read_input("input/2024/sample/day4.txt");
        assert_eq!(part1(&input), 18);
    }
}
