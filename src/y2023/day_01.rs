/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use regex::Regex;
use std::collections::HashMap;

use crate::{Fro, Solution, TaskResult};

// Can add more shared vars here
pub struct Trebuchet {
    data: Vec<String>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for Trebuchet {
    fn fro(input: &str) -> Self {
        Self {
            data: input.split('\n').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for Trebuchet {
    fn silver(&self) -> TaskResult {
        let mut sum: usize = 0;

        for row in &self.data {
            let mut first: char = 'E';
            let mut second: char = 'E';

            for char in row.chars() {
                if char.is_ascii_digit() && first == 'E' {
                    first = char;
                } else if char.is_ascii_digit() {
                    second = char;
                }
            }

            if first != 'E' && second == 'E' {
                second = first;
            }

            sum += format!("{}{}", first, second).parse::<usize>().unwrap();
        }

        TaskResult::Usize(sum)
    }

    fn gold(&self) -> TaskResult {
        let mut sum: usize = 0;

        let terms: HashMap<&str, char> = HashMap::from([
            ("one", '1'),
            ("two", '2'),
            ("three", '3'),
            ("four", '4'),
            ("five", '5'),
            ("six", '6'),
            ("seven", '7'),
            ("eight", '8'),
            ("nine", '9'),
        ]);
        let regex_first =
            Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
        let regex_second =
            Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();

        for row in self.data.iter() {
            //print!("{}: {:?}", i, row);

            // NOT SAFE BUT WHATEVER
            let first_str = regex_first.find(row).unwrap().as_str();
            let reversed_row: String = row.chars().rev().collect();
            let second_str = regex_second.find(&reversed_row).unwrap().as_str();

            let mut first: char = first_str.chars().next().unwrap();
            let mut second: char = second_str.chars().next().unwrap();

            // replace with actual char if needed
            if first_str.len() > 1 {
                first = terms[first_str];
            }
            if second_str.len() > 1 {
                let original_str: String = second_str.chars().rev().collect();
                second = terms[&original_str.as_str()];
            }

            //print!(" {}, {}", first, second);

            let coord = format!("{}{}", first, second).parse::<usize>().unwrap();
            //println!(" = {}", coord);
            sum += coord;
        }
        TaskResult::Usize(sum)
    }
}

// For assisting functions
impl Trebuchet {}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let mut test_data = read_data_from_file("input/2023/test/01a.txt");
        let mut queue = Trebuchet::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(142));

        test_data = read_data_from_file("input/2023/test/01b.txt");
        queue = Trebuchet::fro(&test_data);
        assert_eq!(queue.gold(), TaskResult::Usize(281));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2023/real/01.txt");
        let queue = Trebuchet::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(55017));
        assert_eq!(queue.gold(), TaskResult::Usize(53539));
    }
}
