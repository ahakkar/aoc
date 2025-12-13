/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use crate::{Fro, Solution, TaskResult};

// Can add more shared vars here
pub struct PulsePropagation {
    data: Vec<String>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for PulsePropagation {
    fn fro(input: &str) -> Self {
        Self {
            data: input.split('\n').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for PulsePropagation {
    fn silver(&self) -> TaskResult {
        TaskResult::String("plii".to_string())
    }

    fn gold(&self) -> TaskResult {
        TaskResult::String("plaa".to_string())
    }
}

// For assisting functions
impl PulsePropagation {}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2023/test/20.txt");
        let queue = PulsePropagation::fro(&test_data);

        // test 1 32000000
        // test 2 below:
        assert_eq!(queue.silver(), TaskResult::Usize(11687500));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2023/real/20.txt");
        let queue = PulsePropagation::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }
}
