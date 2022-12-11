use std::io::{self, BufRead};

// type Assignment = ((u8, u8), (u8, u8));
#[derive(Debug)]
pub struct Assignment {
    pub left: (u8, u8),
    pub right: (u8, u8),
}

pub fn solve_part_1() -> i16 {
    let assignments = vec_string_into_tuple(get_input_from_stdin());

    let mut num_contained = 0;

    for a in assignments {
        match contains(a.left, a.right) {
            true => num_contained += 1,
            false => match contains(a.right, a.left) {
                true => num_contained += 1,
                false => (),
            },
        }
    }

    num_contained
}

pub fn solve_part_2() -> i16 {
    let assignments = vec_string_into_tuple(get_input_from_stdin());

    let mut num_contained = 0;

    for a in assignments {
        match overlaps(a.left, a.right) {
            true => num_contained += 1,
            false => match overlaps(a.right, a.left) {
                true => num_contained += 1,
                false => (),
            },
        }
    }

    num_contained
}

fn contains(subject: (u8, u8), other: (u8, u8)) -> bool {
    subject.0 <= other.0 && subject.1 >= other.1
}

fn overlaps(subject: (u8, u8), other: (u8, u8)) -> bool {
    (subject.0 <= other.0 && subject.1 >= other.0) || (subject.0 <= other.1 && subject.1 >= other.1)
}

impl PartialEq for Assignment {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right
    }
}

fn get_input_from_stdin() -> Vec<String> {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let vec = stdin_lock.lines().filter_map(|l| l.ok()).collect();

    vec
}

// input is pair that looks like 2-31 meaning sections 2 thru 31
fn assignment_from_str(s: &str) -> Assignment {
    // this is not very rusty at all... come back to this
    let mut iterable = s
        .split(',')
        .map(|s| s.split('-').map(|n| n.parse::<u8>()))
        .into_iter()
        .flat_map(|f| f);
    let first: u8 = iterable.next().unwrap().unwrap();
    let second: u8 = iterable.next().unwrap().unwrap();
    let third: u8 = iterable.next().unwrap().unwrap();
    let fourth: u8 = iterable.next().unwrap().unwrap();

    Assignment {
        left: (first, second),
        right: (third, fourth),
    }
}

fn vec_string_into_tuple(lines: Vec<String>) -> Vec<Assignment> {
    lines.into_iter().map(|l| assignment_from_str(&l)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_assignment_from_str() {
        let s = String::from("22-89,3-54");
        let result = assignment_from_str(&s);

        assert_eq!(
            Assignment {
                left: (22, 89),
                right: (3, 54)
            },
            result
        );
    }

    #[test]
    fn test_vec_string_into_tuple() {
        let v = vec![String::from("33-99,2-4"), String::from("11-9,29-1")];
        let result = vec_string_into_tuple(v);

        assert_eq!(
            vec![
                Assignment {
                    left: (33, 99),
                    right: (2, 4)
                },
                Assignment {
                    left: (11, 9),
                    right: (29, 1)
                }
            ],
            result
        );
    }

    #[test]
    fn test_tuple_contains_other_tuple() {
        let x = (2, 44);
        let y = (3, 43);

        assert_eq!(true, contains(x, y));

        let x = (33, 36);
        let y = (33, 35);

        assert_eq!(true, contains(x, y));

        let x = (22, 30);
        let y = (27, 30);

        assert_eq!(true, contains(x, y));

        let x = (22, 30);
        let y = (23, 34);

        assert_eq!(false, contains(x, y));
    }

    #[test]
    fn test_tuple_overlaps_other_tuple() {
        let x = (2, 3);
        let y = (4, 5);

        assert_eq!(false, overlaps(x, y));

        let x = (2, 4);
        let y = (6, 8);

        assert_eq!(false, overlaps(x, y));

        let x = (5, 7);
        let y = (7, 9);

        assert_eq!(true, overlaps(x, y));

        let x = (2, 8);
        let y = (3, 7);

        assert_eq!(true, overlaps(x, y));

        let x = (6, 6);
        let y = (4, 6);

        assert_eq!(true, overlaps(x, y));

        let x = (2, 6);
        let y = (4, 8);

        assert_eq!(true, overlaps(x, y));
    }
}
