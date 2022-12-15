use std::io;
use std::io::BufRead;

use day6::{solve_part_1, solve_part_2};

fn main() {
    let input = input_from_stdin();
    let solution = solve_part_2(&input[0]);
    println!("{}", solution);
}

fn input_from_stdin() -> Vec<String> {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let vec = stdin_lock.lines().filter_map(|l| l.ok()).collect();

    vec
}
