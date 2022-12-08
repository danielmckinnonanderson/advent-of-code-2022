use std::collections::HashSet;
use std::io;
use std::io::prelude::*;
use std::str;

pub fn solve_part_1() -> i32 {
    let input = get_input_from_stdin();

    let mut sum: i32 = 0;

    for line in input.iter() {
        let split = split_content_in_half(&line);
        let matches: HashSet<char> = check_for_matches(split);
        for char in matches {
            sum += priority_from_char(&char);
        }
    }

    sum
}

pub fn solve_part_2() -> i32 {
    let input = get_input_from_stdin();

    let mut sum: i32 = 0;

    let mut group_of_three: Vec<&str> = vec![];

    for line in input.iter() {
        group_of_three.push(line);

        if group_of_three.len() == 3 {
            let matches = check_for_matches(group_of_three);

            for char in matches {
                let priority = priority_from_char(&char);
                sum += priority;
            }

            group_of_three = vec![];
        }
    }

    sum
}

fn get_input_from_stdin() -> Vec<String> {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let vec = stdin_lock.lines().filter_map(|l| l.ok()).collect();

    vec
}

fn split_content_in_half(content: &str) -> Vec<&str> {
    let length = content.len();

    let midpoint = length / 2;

    let split = content.split_at(midpoint);

    vec![split.0, split.1]
}

fn check_for_matches(comparators: Vec<&str>) -> HashSet<char> {
    let mut matches: HashSet<char> = HashSet::new();

    for char in comparators.first().unwrap().chars() {
        if matches.contains(&char) {
            continue;
        }

        for char_1 in comparators.get(1).unwrap().chars() {
            if char != char_1 {
                continue;
            }

            // I rewrote my solution from pt 1 to be used here thinking it'd be 
            // smart and elegant. instead it looks like shit
            if comparators.len() == 2 {
                matches.insert(char);
                break;

            } else if comparators.len() == 3 {
                for char_2 in comparators.get(2).unwrap().chars() {
                    if char_1 != char_2 {
                        continue;
                    }
                    matches.insert(char);
                    break;
                }
            }
        }
    }

    matches
}

fn priority_from_char(c: &char) -> i32 {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => panic!("Invalid input character! Character was {}", c),
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_split_content_in_half() {
        let even = String::from("aaaaazzzzz");
        let split = split_content_in_half(&even);
        assert_eq!(5, split.first().unwrap().len());
        assert_eq!(5, split.get(1).unwrap().len());

        let odd = String::from("bbbbxxxxx");
        let split = split_content_in_half(&odd);
        assert_eq!(4, split.first().unwrap().len());
        assert_eq!(5, split.get(1).unwrap().len());
    }

    #[test]
    fn test_checking_for_matches() {
        let compartment_a = String::from("gGdeqGlPfz");
        let compartment_b = String::from("eqkdoGsGGG");

        let result: HashSet<char> = check_for_matches(vec![&compartment_a, &compartment_b]);

        println!("{:?}", result);
        assert_eq!(4, result.len());
        assert_eq!('e', *result.get(&'e').unwrap());
        assert_eq!('G', *result.get(&'G').unwrap());
        assert_eq!('d', *result.get(&'d').unwrap());
        assert_eq!('q', *result.get(&'q').unwrap());
    }

    #[test]
    fn test_part_two() {
        let str_1 = String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
        let str_2 = String::from("ttgJtRGJQctTZtZT");
        let str_3 = String::from("CrZsJsPPZsGzwwsLwLmpwMDw");

        let matches: HashSet<char> = check_for_matches(vec![&str_1, &str_2, &str_3]);

        assert_eq!(1, matches.len());
        assert!(matches.contains(&'Z'));

        let value = priority_from_char(&'Z');
        assert_eq!(52, value);
    }
}
