use std::fs;
use day1::top_3_calories;


fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    println!("{}", top_3_calories(&input).sum());
}

