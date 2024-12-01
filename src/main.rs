use std::{fs, iter::zip};

fn main() {
    let data = fs::read_to_string("inputs/day1p1_sample.txt").expect("Unable to read input!");
    let result = day1p1(data.trim());
    println!("{}", result);
}

fn day1p1(input: &str) -> isize {
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
