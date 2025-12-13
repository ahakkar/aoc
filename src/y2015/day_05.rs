/*
 * 2015 Advent of Code with Rust
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

use crate::{Fro, Solution, TaskResult};

// Can add more shared vars here
pub struct DoesntHeHaveInternElvesForThis {
    data: Vec<String>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for DoesntHeHaveInternElvesForThis {
    fn fro(input: &str) -> Self {
        Self {
            data: input.split('\n').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for DoesntHeHaveInternElvesForThis {
    fn silver(&self) -> TaskResult {
        let mut sum = 0;

        for string in &self.data {
            let mut double = false;
            let mut wovels = 0;
            let mut invalid = false;

            for w in string.as_bytes().windows(2) {
                let c1 = w[0] as char;
                let c2 = w[1] as char;

                if DoesntHeHaveInternElvesForThis::is_invalid(c1, c2) {
                    invalid = true;
                    break;
                }

                if matches!(c1, 'a' | 'e' | 'i' | 'o' | 'u') {
                    wovels += 1;
                }

                if c1 == c2 {
                    double = true;
                }
            }

            // Check last char
            if matches!(string.chars().last().unwrap(), 'a' | 'e' | 'i' | 'o' | 'u') {
                wovels += 1;
            }
            if invalid {
                continue;
            } else if double && wovels > 2 {
                sum += 1;
            }
        }
        // 329 too high
        TaskResult::Usize(sum)
    }

    fn gold(&self) -> TaskResult {
        TaskResult::String("plaa".to_string())
    }
}

// For assisting functions
impl DoesntHeHaveInternElvesForThis {
    // ab, cd, pq, xy
    fn is_invalid(c1: char, c2: char) -> bool {
        if (c1 == 'a' && c2 == 'b')
            || (c1 == 'c' && c2 == 'd')
            || (c1 == 'p' && c2 == 'q')
            || (c1 == 'x' && c2 == 'y')
        {
            return true;
        }
        false
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2015/test/05.txt");
        let queue = DoesntHeHaveInternElvesForThis::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(1));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2015/real/05.txt");
        let queue = DoesntHeHaveInternElvesForThis::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(236));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }
}
