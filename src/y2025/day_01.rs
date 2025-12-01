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

use crate::{Fro, Solution, TaskResult};

// Can add more shared vars here
pub struct SecretEntrance {
    data: Vec<String>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for SecretEntrance {
    fn fro(input: &str) -> Self {
        Self {
            data: input.split('\n').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for SecretEntrance {
    fn silver(&self) -> TaskResult {
        let mut result = 0;

        for row in &self.data {
            println!("dir: {}, amount: {}", &row[0..1], &row[1..]);
            match &row[0..1] {
                "L" => (),
                "R" => (),
                _ => panic!("not expected direction"),
            }
        }

        TaskResult::Usize(result)
    }

    fn gold(&self) -> TaskResult {
        TaskResult::Usize(232)
    }
}

// For assisting functions
impl SecretEntrance {
    fn wrapping_add(a: &i32, b: &i32, lo: &i32, hi: &i32) -> i32 {
        *a
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2025/test/01.txt");
        let queue = SecretEntrance::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2025/real/01.txt");
        let queue = SecretEntrance::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }
}
