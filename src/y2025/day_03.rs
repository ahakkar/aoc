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
        self.data
            .iter()
            .map(|line| {
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

                (a * 10 + b) as usize
            })
            .sum::<usize>()
            .into()
    }

    fn gold(&self) -> TaskResult {
        // Monotonic decreasing stack
        self.data
            .iter()
            .map(|line| {
                let mut stack: Vec<usize> = vec![];
                let digits: Vec<usize> = line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
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
                stack.iter().fold(0, |acc, n| acc * 10 + n)
            })
            .sum::<usize>()
            .into()
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
