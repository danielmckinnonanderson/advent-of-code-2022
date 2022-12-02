use std::fs;
use day1::calorie_counting;


fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    println!("{}", calorie_counting(&input));
}

