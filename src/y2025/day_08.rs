/*
 * 2025 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

#![allow(dead_code)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::only_used_in_recursion)]
#![allow(clippy::never_loop)]
#![allow(clippy::useless_vec)]
#![allow(clippy::collapsible_if)]

use crate::{Fro, Solution, TaskResult, util::point3d::Point3d};

// Can add more shared vars here
pub struct Playground {
    data: Vec<Point3d>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for Playground {
    fn fro(input: &str) -> Self {
        Self {
            // Discards bad input
            data: input
                .split('\n')
                .filter_map(Point3d::new_from_str)
                .collect(),
        }
    }
}

// Main solvers
impl Solution for Playground {
    fn silver(&self) -> TaskResult {
        println!("{:?}, len: {}", self.data, self.data.len());
        TaskResult::String("plii".to_string())
    }

    fn gold(&self) -> TaskResult {
        TaskResult::String("plaa".to_string())
    }
}

// For assisting functions
impl Playground {}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2025/test/08.txt");
        let queue = Playground::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2025/real/08.txt");
        let queue = Playground::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }
}
