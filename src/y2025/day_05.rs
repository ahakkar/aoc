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

use std::cmp::{Ordering, max};

use crate::{Fro, Solution, TaskResult};

// Can add more shared vars here
pub struct Cafeteria {
    merged: Vec<(usize, usize)>,
    ids: Vec<usize>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for Cafeteria {
    fn fro(input: &str) -> Self {
        let mut toggle = true;
        let mut ranges: Vec<(usize, usize)> = vec![];
        let mut ids: Vec<usize> = vec![];

        // parse data
        for row in input.split('\n') {
            if row.is_empty() {
                toggle = false;
                continue;
            }
            if toggle {
                let (a, b) = row
                    .split_once('-')
                    .map(|(s1, s2)| {
                        (s1.parse::<usize>().unwrap(), s2.parse::<usize>().unwrap())
                    })
                    .unwrap();
                ranges.push((a, b));
            } else {
                ids.push(row.parse::<usize>().unwrap());
            }
        }

        ranges.sort_by_key(|&(a, _)| a);

        // merge sorted ranges
        let mut merged: Vec<(usize, usize)> = vec![];

        for range in ranges {
            if let Some(last) = merged.last_mut() {
                if range.0 <= last.1 {
                    last.1 = max(last.1, range.1);
                } else {
                    merged.push(range);
                }
            } else {
                merged.push(range);
            }
        }

        Self { merged, ids }
    }
}

// Main solvers
impl Solution for Cafeteria {
    fn silver(&self) -> TaskResult {
        self.ids
            .iter()
            .fold(0, |acc, id| {
                if self
                    .merged
                    .binary_search_by(|&(start, end)| {
                        if *id > end {
                            Ordering::Less
                        } else if *id < start {
                            Ordering::Greater
                        } else {
                            Ordering::Equal
                        }
                    })
                    .is_ok()
                {
                    return acc + 1;
                }
                acc
            })
            .into()
    }

    fn gold(&self) -> TaskResult {
        self.merged
            .iter()
            .fold(0, |acc, range| acc + (range.1 - range.0 + 1))
            .into()
    }
}

// For assisting functions
impl Cafeteria {}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2025/test/05.txt");
        let queue = Cafeteria::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(3));
        assert_eq!(queue.gold(), TaskResult::Usize(14));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2025/real/05.txt");
        let queue = Cafeteria::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(635));
        assert_eq!(queue.gold(), TaskResult::Usize(369761800782619));
    }
}
