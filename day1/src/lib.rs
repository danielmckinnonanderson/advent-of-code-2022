// given a list of integers where values are separated by line
// and groups are separated by a line break

// find the group whose values produce the largest sum and return that sum

pub fn calorie_counting(input: &str) -> i32 {
    // initialize result
    let mut largest = 0;

    // iterate over each line
    let iter = input.split("\n");

    let mut current_sum = 0;

    for s in iter {
        if s == "" {
            // if line is empty, check current sum against largest
            // replace largest with current_sum if current_sum is larger
            if current_sum > largest {
                largest = current_sum;
                println!("Updated largest! Value now {}", largest);
            }
            // reset current_sum prior to iterating
            current_sum = 0;

        } else {
            // if line is a value, convert it to number and add to current sum
            let value = s.parse::<i32>().unwrap();
            current_sum += value;
        }
    }

    largest
}

pub struct TopThree {
    first: i32,
    second: i32,
    third: i32,
}

impl TopThree {
    pub fn update_first(&mut self, value: i32) {
        self.third = self.second;
        self.second = self.first;
        self.first = value;
    }

    pub fn update_second(&mut self, value: i32) {
        // if self.second == 0 {
            // self.second = value;
        // } else {
            self.third = self.second;
            self.second = value;
        // }

    }

    pub fn update_third(&mut self, value: i32) {
        self.third = value;
    }

    pub fn sum(&self) -> i32 {
        self.first + self.second + self.third
    }
}

pub fn top_3_calories(input: &str) -> TopThree {
    // initialize result 
    let mut result = TopThree {
        first: 2,
        second: 1,
        third: 0
    };

    let iter = input.split("\n");

    let mut current_sum = 0;

    for s in iter {
        if s == "" {
            if current_sum > result.third {
                if current_sum > result.second {
                    if current_sum > result.first {
                        println!("first was {}, is now {}", result.first, current_sum);
                        result.update_first(current_sum);
                    } else {
                        println!("second was {}, is now {}", result.second, current_sum);
                        result.update_second(current_sum);
                    }
                } else{ 
                    println!("third was {}, is now {}", result.third, current_sum);
                    result.update_third(current_sum);
                }
            }
            current_sum = 0;
        } else {
            let value = s.parse::<i32>().unwrap();
            current_sum += value;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn returns_largest_sum() {
        let input = String::from("1000\n2000\n2000\n\n4000\n6000\n\n3000\n\n2000\n\n");

        let result = calorie_counting(&input);

        assert_eq!(10000, result);
    }

    #[test]
    fn returns_sum_of_top_3() {
        let input = String::from("1000\n2000\n2000\n\n4000\n6000\n\n3000\n\n2000\n\n120000\n\n11000\n9000\n4500\n\n");

        let result = top_3_calories(&input);

        assert_eq!(120000, result.first);
        assert_eq!(24500, result.second);
        assert_eq!(10000, result.third);

        assert_eq!(154500, result.sum());
    }
}
