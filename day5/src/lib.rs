use std::{collections::HashMap, io::Error};
use std::io::{self, BufRead}; 

pub fn solve_part_1() -> Vec<String> {
    let input: Vec<String> = get_input_from_stdin();

    let input_diagram = &input[0..9].to_vec();
    let instructions = &input[10..].to_vec();

    let mut arrangement = arrangement_from_input_diagram(&input_diagram);

    arrangement.execute_instructions(instructions, &MoveStrategy::OneByOne);

    let mut result: Vec<String> = vec![];
    
    let mut i = 1;

    while i < 10 {
        result.push(arrangement.peek(i).unwrap().0.clone());
        i += 1;
    }

    result
}

pub fn solve_part_2() -> Vec<String> {
    let input: Vec<String> = get_input_from_stdin();

    let input_diagram = &input[0..9].to_vec();
    let instructions = &input[10..].to_vec();

    let mut arrangement = arrangement_from_input_diagram(&input_diagram);

    arrangement.execute_instructions(instructions, &MoveStrategy::AsStack);

    let mut result: Vec<String> = vec![];
    
    let mut i = 1;

    while i < 10 {
        result.push(arrangement.peek(i).unwrap().0.clone());
        i += 1;
    }

    result
}

fn get_input_from_stdin() -> Vec<String> {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let vec = stdin_lock.lines().filter_map(|l| l.ok()).collect();

    vec
}


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Crate(String);

impl Crate {
    pub fn from_input_diagram_str(s: &str) -> Crate {
        Crate(
            // String in form "[V]"
            s.replace(&['[', ']'][..], ""),
        )
    }

    pub fn new(s: String) -> Crate {
        Crate(s)
    }
}

type Arrangement = HashMap<i8, Vec<Crate>>;

enum MoveStrategy {
    OneByOne,
    AsStack
}

fn arrangement_from_input_diagram(input: &Vec<String>) -> Arrangement {
    let mut result: Arrangement = HashMap::new();

    // collect rows and split them into columns instead
    // this is so sickening and vile and horrible
    for row in input {
        let items = sub_strings(&row, 4);

        // if we reach the row of numbers, we're done here
        if items.first().unwrap().contains("1") {
            break;
        }

        let mut i: usize = 1;

        while i < 10 {
            if !result.contains_key(&(i as i8)) {
                result.insert(i as i8, vec![]);
            }
            
            if !items.get(i - 1).unwrap().trim().is_empty() {
                let item = items.get(i -1).unwrap()
                                      .trim()
                                      .trim_end_matches(']')
                                      .trim_start_matches('[')
                                      .to_string();
                
                result.prepend_insert_into(i as i8, Crate::new(item));
            }
            i += 1;
        }
    }
    result
}

fn sub_strings(s: &str, sub_len: usize) -> Vec<&str> {
    let mut subs = vec![];
    let mut iter = s.chars();
    let mut pos = 0;

    while pos < s.len() {
        let mut len = 0;
        for c in iter.by_ref().take(sub_len) {
            len += c.len_utf8();
        }

        subs.push(&s[pos..pos + len]);
        pos += len;
    }
    subs
}

trait CrateArrangment {
    fn pop(&mut self, index: i8, quantity: i8) -> Result<Vec<Crate>, Error>;

    fn peek(&self, index: i8) -> Result<&Crate, Error>;

    fn move_crates_one_by_one(&mut self, index_from: i8, index_to: i8, quantity: i8);

    fn move_crates_as_stack(&mut self, index_from: i8, index_to:i8, quantity: i8);

    fn insert_into(&mut self, index: i8, value: Crate);

    fn prepend_insert_into(&mut self, key: i8, value: Crate);

    fn execute_instruction(&mut self, instruction: &str, strategy: &MoveStrategy);

    fn execute_instructions(&mut self, instructions: &Vec<String>, strategy: &MoveStrategy);
}

impl CrateArrangment for Arrangement {
    fn pop(&mut self, index: i8, quantity: i8) -> Result<Vec<Crate>, Error> {
        match self.get_mut(&index) {
            Option::Some(vec) => {
                // println!("Vector at index {} is {:?}, will attempt to move {}", index, vec, quantity);
                let mut result: Vec<Crate> = vec![];
                let mut i = 0;

                while i < quantity {
                    match vec.pop() {
                        Option::Some(c) => result.push(c),
                        Option::None => panic!(
                            "Invalid quantity, quantity was {} but vector had length {}",
                            quantity,
                            vec.len()
                        ),
                    }

                    i += 1;
                }

                Ok(result)
            }
            Option::None => Result::Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Index was not found in map, index was {}", index),
            )),
        }
    }

    fn peek(&self, index: i8) -> Result<&Crate, Error> {
        match self.contains_key(&index) {
            true => match self.get(&index).unwrap().last() {
                Some(value) => Ok(&value),
                None => Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Stack at index {} was empty", index)
                ))
            }
            false => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Index was not found in map, index was {}", index)
            ))
        }
    }

    fn move_crates_one_by_one(&mut self, index_from: i8, index_to: i8, quantity: i8) {
        match self.pop(index_from, quantity) {
            Ok(moving) => match self.get_mut(&index_to) {
                Some(vec) => {
                    let moved = vec![vec.to_vec(), moving].concat();
                    self.insert(index_to, moved);
                }
                None => panic!("Stinky"),
            },
            Err(_) => panic!("Bad"),
        }
    }

    fn move_crates_as_stack(&mut self, index_from: i8, index_to:i8, quantity: i8) {
        match self.pop(index_from, quantity) {
            Ok(mut moving) => match self.get_mut(&index_to) {
                Some(vec) => {
                    moving.reverse();
                    let moved = vec![vec.to_vec(), moving].concat();
                    self.insert(index_to, moved);
                }
                None => panic!("Stinky")
            },
                Err(_) => panic!("Bad")
        }
    }

    fn insert_into(&mut self, key: i8, value: Crate) {
        let v: &Vec<Crate> = self.get(&key).unwrap();
        let v = vec![v.to_vec(), vec![value]].concat();
        self.insert(key, v);
    }

    fn prepend_insert_into(&mut self, key: i8, value: Crate) {
        let v: &Vec<Crate> = self.get(&key).unwrap();
        // this is disgusting
        let v = vec![vec![value], v.to_vec()].concat();
        self.insert(key, v);
    }

    fn execute_instruction(&mut self, instruction: &str, strategy: &MoveStrategy) {
        // instruction in form of "move x from y to z"
        let instruction = instruction.split_whitespace().collect::<Vec<&str>>();
        let quantity = instruction.get(1).unwrap().parse::<i8>().unwrap();
        let index_from = instruction.get(3).unwrap().parse::<i8>().unwrap();
        let index_to = instruction.get(5).unwrap().parse::<i8>().unwrap();

        match strategy {
            MoveStrategy::OneByOne => self.move_crates_one_by_one(index_from, index_to, quantity),
            MoveStrategy::AsStack => self.move_crates_as_stack(index_from, index_to, quantity)
        }
    }

    fn execute_instructions(&mut self, instructions: &Vec<String>, strategy: &MoveStrategy) {
        for instruction in instructions {
            self.execute_instruction(instruction, strategy);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_from_str() {
        let s = String::from("[X]");
        let result = Crate::from_input_diagram_str(&s);

        assert_eq!(result, Crate(String::from("X")));
    }

    #[test]
    fn test_crate_arr_pop() {
        let mut a: Arrangement = HashMap::new();

        a.insert(
            1,
            vec![
                Crate(String::from("V")),
                Crate(String::from("S")),
                Crate(String::from("A")),
                Crate(String::from("R")),
            ],
        );

        a.insert(
            2,
            vec![
                Crate(String::from("X")),
                Crate(String::from("Z")),
                Crate(String::from("E")),
            ],
        );

        let result = a.pop(2, 2);

        assert_eq!(
            vec![Crate(String::from("E")), Crate(String::from("Z"))],
            result.unwrap()
        );

        assert_eq!(1, a.get(&2).unwrap().len());
        assert_eq!(Crate("X".to_string()), a.get(&2).unwrap()[0]);
    }

    #[test]
    fn test_crate_arr_move() {
        let mut a: Arrangement = HashMap::new();

        a.insert(
            1,
            vec![
                Crate(String::from("V")),
                Crate(String::from("S")),
                Crate(String::from("A")),
                Crate(String::from("R")),
            ],
        );

        a.insert(
            2,
            vec![
                Crate(String::from("X")),
                Crate(String::from("Z")),
                Crate(String::from("E")),
            ],
        );

        a.move_crates_one_by_one(2, 1, 1);

        assert_eq!(
            vec![
                Crate(String::from("V")),
                Crate(String::from("S")),
                Crate(String::from("A")),
                Crate(String::from("R")),
                Crate(String::from("E")),
            ],
            *a.get(&1).unwrap(),
        );

        assert_eq!(
            vec![Crate(String::from("X")), Crate(String::from("Z")),],
            *a.get(&2).unwrap(),
        );
    }

    #[test]
    fn test_arr_from_input() {
        let input = vec![
            String::from("                                   "),
            String::from("            [W]                    "),
            String::from("[P]         [R]     [G] [I] [T]    "),
            String::from("[M]     [O] [Z]     [G] [U] [D]    "),
            String::from("[J]     [L] [K] [E] [U] [S] [A] [E]"),
            String::from("[G] [H] [I] [G] [P] [N] [H] [W] [Q]"),
            String::from("[D] [E] [F] [E] [Q] [U] [C] [B] [B]"),
            String::from("[A] [B] [A] [A] [U] [X] [Y] [P] [O]"),
            String::from(" 1   2   3   4   5   6   7   8   9"),
        ];

        let result = arrangement_from_input_diagram(&input);

        assert_eq!(
            vec![
                Crate(String::from("A")),
                Crate(String::from("D")),
                Crate(String::from("G")),
                Crate(String::from("J")),
                Crate(String::from("M")),
                Crate(String::from("P")),
            ],
            *result.get(&1).unwrap()
        );

        assert_eq!(
            vec![
                Crate(String::from("B")),
                Crate(String::from("E")),
                Crate(String::from("H")),
            ],
            *result.get(&2).unwrap()
        );

        assert_eq!(
            vec![
                Crate(String::from("A")),
                Crate(String::from("F")),
                Crate(String::from("I")),
                Crate(String::from("L")),
                Crate(String::from("O")),
            ],
            *result.get(&3).unwrap()
        );

        assert_eq!(
            vec![
                Crate(String::from("A")),
                Crate(String::from("E")),
                Crate(String::from("G")),
                Crate(String::from("K")),
                Crate(String::from("Z")),
                Crate(String::from("R")),
                Crate(String::from("W")),
            ],
            *result.get(&4).unwrap()
        );
    }

    #[test]
    fn test_execute_instructions() {
        let mut a: Arrangement = HashMap::new();

        a.insert(
            1,
            vec![
                Crate::new("A".to_string()),
                Crate::new("D".to_string()),
                Crate::new("G".to_string()),
                Crate::new("J".to_string()),
                Crate::new("M".to_string()),
                Crate::new("P".to_string()),
            ],
        );

        a.insert(
            2,
            vec![
                Crate::new("B".to_string()),
                Crate::new("E".to_string()),
                Crate::new("H".to_string()),
            ],
        );

        a.insert(
            3,
            vec![
                Crate::new("C".to_string()),
                Crate::new("F".to_string()),
                Crate::new("I".to_string()),
                Crate::new("M".to_string()),
                Crate::new("A".to_string()),
            ],
        );

        let input = vec![
            String::from("move 1 from 1 to 3"),
            String::from("move 2 from 2 to 3"),
            String::from("move 4 from 3 to 1"),
        ];

        a.execute_instructions(&input, &MoveStrategy::OneByOne);

        assert_eq!(9, a.get(&1).unwrap().len());
        assert_eq!(1, a.get(&2).unwrap().len());
        assert_eq!(4, a.get(&3).unwrap().len());
    }
}
