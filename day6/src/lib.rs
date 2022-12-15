use std::collections::HashSet;

pub fn solve_part_1(input: &str) -> usize {
    let mut b = Buffer::new(input);

    let mut is_unique = false;

    while !is_unique {
        let str = b.read_quantity_at_index(4);
        match all_unique(str) {
            true => {
                is_unique = true
            },
            false => {
                b.cycle();
            }
        }
    }

    b.index + 4
}

pub fn solve_part_2(input: &str) -> usize {
    let mut b = Buffer::new(input);

    let mut is_unique = false;

    while !is_unique {
        let str = b.read_quantity_at_index(14);
        match all_unique(str) {
            true => {
                is_unique = true
            },
            false => {
                b.cycle();
            }
        }
    }

    b.index + 14
}

pub struct Buffer<'a> {
    input: &'a str,
    index: usize,
}

impl<'a> Buffer<'a> {
    pub fn new(s: &'a str) -> Buffer<'a> {
        Buffer { input: s, index: 0 }
    }

    pub fn cycle(&mut self) {
        self.index += 1;
    }

    pub fn read_quantity_at_index(&self, quantity: usize) -> &str {
        let end = quantity + self.index;
        &self.input[self.index..end]
    }
}

pub fn all_unique(s: &str) -> bool {
    
    let mut chars = s.chars().into_iter();
    let mut encountered: HashSet<char> = HashSet::new();

    let mut i = 0;

    while i < s.len() {
        let c = chars.next().unwrap();
        if encountered.insert(c) == false {
            return false;
        }
        i += 1;
    }

    true
}

#[cfg(test)]
mod test {
    use crate::all_unique;

    #[test]
    fn test_unique() {
        let s = String::from("svfcsfzls");
        assert_eq!(true, all_unique(&s[..4]));
        assert_eq!(false, all_unique(&s[2..6]));
    }
}
