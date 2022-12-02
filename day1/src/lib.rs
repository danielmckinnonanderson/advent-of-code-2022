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

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn returns_largest_sum() {
        let input = String::from("1000\n2000\n2000\n\n4000\n6000\n\n3000\n\n2000");

        let result = calorie_counting(&input);

        assert_eq!(10000, result);
    }
}
