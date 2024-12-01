use std::fs;

fn main() {
    let data = fs::read_to_string("inputs/day1p1_sample.txt").expect("Unable to read input!");
    println!("{}", data);
}
