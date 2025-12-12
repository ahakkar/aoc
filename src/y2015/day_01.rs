/*
 * 2015 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use crate::{Fro, Solution, TaskResult};

// Can add more shared vars here
pub struct NotQuiteLisp {
    data: String,
}

// Can be used to implement fancier task-specific parsing
impl Fro for NotQuiteLisp {
    fn fro(input: &str) -> Self {
        Self {
            data: input.split('\n').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for NotQuiteLisp {
    fn silver(&self) -> TaskResult {
        let mut up = 0;
        let mut down = 0;

        self.data.chars().for_each(|c| match c {
            '(' => up += 1,
            ')' => down += 1,
            _ => panic!(),
        });
        TaskResult::Usize(up - down)
    }

    fn gold(&self) -> TaskResult {
        let mut up: i64 = 0;
        let mut down: i64 = 0;
        let mut index = 0;

        for (i, c) in self.data.chars().enumerate() {
            match c {
                '(' => up += 1,
                ')' => down += 1,
                _ => panic!(),
            }
            if up - down == -1 {
                index = i;
                break;
            }
        }
        // first floor is 1 instead of 0 so need to adjust index
        TaskResult::Usize(index + 1)
    }
}

// For assisting functions
impl NotQuiteLisp {}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2025/test/01.txt");
        let queue = NotQuiteLisp::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2025/real/01.txt");
        let queue = NotQuiteLisp::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(138));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }
}
