/*
 * 2025 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use crate::{Fro, Solution, TaskResult};

// Can add more shared vars here
pub struct Lobby {
    data: Vec<String>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for Lobby {
    fn fro(input: &str) -> Self {
        Self {
            data: input.split('\n').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for Lobby {
    fn silver(&self) -> TaskResult {
        let mut result = 0;

        for line in &self.data {
            let mut a = 0;
            let mut b = 0;

            for (idx, chr) in line.chars().enumerate() {
                let n = chr.to_digit(10).unwrap();

                if n > a && idx < line.len() - 1 {
                    a = n;
                    b = 0;
                } else if n > b {
                    b = n;
                }
            }

            let nums = format!("{}{}", a, b).parse::<usize>().unwrap();
            // println!("line: {}\nnums: {}", line, nums);
            result += nums;
        }

        TaskResult::Usize(result)
    }

    fn gold(&self) -> TaskResult {
        let mut gold = 0;

        // Monotonic decreasing stack
        for line in &self.data {
            let mut stack: Vec<i32> = vec![];
            let digits: Vec<i32> = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect();

            let mut to_remove = digits.len() - 12;

            for d in digits {
                while let Some(&last) = stack.last() {
                    if to_remove > 0 && last < d {
                        stack.pop();
                        to_remove -= 1;
                    } else {
                        break;
                    }
                }
                stack.push(d);
            }

            stack.truncate(12);

            gold += stack
                .into_iter()
                .map(|n| n.to_string())
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
        }

        TaskResult::Usize(gold)
    }
}

// For assisting functions
impl Lobby {}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2025/test/03.txt");
        let queue = Lobby::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(357));
        assert_eq!(queue.gold(), TaskResult::Usize(3121910778619));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2025/real/03.txt");
        let queue = Lobby::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(17207));
        assert_eq!(queue.gold(), TaskResult::Usize(170997883706617));
    }
}
