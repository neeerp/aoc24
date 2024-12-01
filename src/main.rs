use std::{fs, iter::zip};

fn main() {
    let day1p1_result = day1p1(read_input("inputs/day1p1_input.txt"));
    println!("The result for day 1 part 1 is: {}", day1p1_result);
}

fn day1p1(input: String) -> isize {
    let (mut first, mut second): (Vec<isize>, Vec<isize>) = input.split('\n').fold((vec![], vec![]), |(mut first, mut second), line| {
        let parts: Vec<&str> = line.split("   ").collect();
        match parts[..] {
            [a, b] => {
                first.push(a.parse().unwrap());
                second.push(b.parse().unwrap());
                (first, second)
            }
            _ => panic!("Oops!")
        }
    });

    first.sort();
    second.sort();

    let sum: isize = zip(first, second).map(|(a, b)| (a - b).abs()).sum();
    sum
}

fn read_input(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Unable to read input!").trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_on_d1p1_sample() {
        let input = read_input("inputs/day1p1_sample.txt");
        assert_eq!(day1p1(input), 11);
    }

    #[test]
    fn it_works_on_d1p1_actual() {
        let input = read_input("inputs/day1p1_input.txt");
        assert_eq!(day1p1(input), 2756096);
    }
}
