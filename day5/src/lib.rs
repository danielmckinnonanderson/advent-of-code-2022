use std::{collections::HashMap, io::Error};
use std::io::{self, BufRead}; 

pub fn solve_part_1() -> Vec<String> {
    let input: Vec<String> = get_input_from_stdin();

    let input_diagram = &input[0..9].to_vec();
    let instructions = &input[10..].to_vec();

    let mut arrangement = arrangement_from_input_diagram(&input_diagram);

    arrangement.execute_instructions(instructions);

    let mut result: Vec<String> = vec![];

    for key in arrangement.keys() {
        result.push(arrangement.peek(*key).unwrap().0.clone());
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

fn arrangement_from_input_diagram(input: &Vec<String>) -> Arrangement {
    let mut result: Arrangement = HashMap::new();

    // hard-code, 9th row of input contains the integer column names
    // set up map key values
    for index in input.get(8).unwrap().split_whitespace().into_iter() {
        result.insert(index.parse::<i8>().unwrap(), vec![]);
    }

    // gross
    let mut input = input.clone();
    input.remove(8);

    // input is 8 rows of crate 'stacks', and 9th row is stack key
    for row in input {
        // divide row into groups of characters, separated by whitespace
        let columns = sub_strings(&row, 4);

        for col in columns.iter() {
            // if the string is not empty, find its column index
            if col.trim() != "" {
                println!("Col = {}", col.trim());
                
                match columns.iter().position(|&s| &s == col) {
                    Some(i) => {
                        println!("Index {}", i);
                        let index: i8 = i as i8 + 1;
                        result.prepend_insert(index, Crate::from_input_diagram_str(&col.trim()));
                        // swap character in input to prevent repeat inserts
                        // columns[i] = &String::from("[9]")
                    }
                    None => (),
                }
            }
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

    fn move_crates(&mut self, index_from: i8, index_to: i8, quantity: i8);

    fn prepend_insert(&mut self, key: i8, value: Crate);

    fn execute_instruction(&mut self, instruction: &str);

    fn execute_instructions(&mut self, instructions: &Vec<String>);
}

impl CrateArrangment for Arrangement {
    fn pop(&mut self, index: i8, quantity: i8) -> Result<Vec<Crate>, Error> {
        match self.get_mut(&index) {
            Option::Some(vec) => {
                println!("Vector at index {} is {:?}, will attempt to move {}", index, vec, quantity);
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
            // true => Ok(self.get(&index).unwrap().last().unwrap()),
            false => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Index was not found in map, index was {}", index)
            ))
        }
    }

    fn move_crates(&mut self, index_from: i8, index_to: i8, quantity: i8) {
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

    fn prepend_insert(&mut self, key: i8, value: Crate) {
        let v: &Vec<Crate> = self.get(&key).unwrap();
        // this is disgusting
        let v = vec![vec![value], v.to_vec()].concat();
        self.insert(key, v);
    }

    fn execute_instruction(&mut self, instruction: &str) {
        // instruction in form of "move x from y to z"
        let instruction = instruction.split_whitespace().collect::<Vec<&str>>();
        let quantity = instruction.get(1).unwrap().parse::<i8>().unwrap();
        let index_from = instruction.get(3).unwrap().parse::<i8>().unwrap();
        let index_to = instruction.get(5).unwrap().parse::<i8>().unwrap();

        self.move_crates(index_from, index_to, quantity);
    }

    fn execute_instructions(&mut self, instructions: &Vec<String>) {
        for instruction in instructions {
            self.execute_instruction(instruction);
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

        a.move_crates(2, 1, 1);

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
            String::from("               "),
            String::from("            [W]"),
            String::from("[P]         [R]"),
            String::from("[M]     [O] [Z]"),
            String::from("[J]     [L] [K]"),
            String::from("[G] [H] [I] [G]"),
            String::from("[D] [E] [F] [E]"),
            String::from("[A] [B] [A] [A]"),
            String::from(" 1   2   3   4 "),
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

        a.execute_instructions(&input);

        assert_eq!(9, a.get(&1).unwrap().len());
        assert_eq!(1, a.get(&2).unwrap().len());
        assert_eq!(4, a.get(&3).unwrap().len());
    }
}
