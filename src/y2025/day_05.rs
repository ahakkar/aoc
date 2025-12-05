/*
 * 2025 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use crate::{Fro, Solution, TaskResult};
use std::cmp::{Ordering, max};

// Can add more shared vars here
pub struct Cafeteria {
    merged: Vec<(usize, usize)>,
    ids: Vec<usize>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for Cafeteria {
    fn fro(input: &str) -> Self {
        let mut iter = input.split("\n\n");

        // parse data
        let mut ranges: Vec<(usize, usize)> = iter
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let (a, b) = line.split_once('-').unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect();
        ranges.sort_by_key(|&(a, _)| a);

        let ids: Vec<usize> = iter
            .next()
            .unwrap()
            .lines()
            .map(|id| id.parse::<usize>().unwrap())
            .collect();

        let mut merged: Vec<(usize, usize)> = vec![];
        for range in ranges {
            if let Some(last) = merged.last_mut()
                && range.0 <= last.1
            {
                last.1 = max(last.1, range.1);
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
            .fold(0, |acc, id| match self.is_id_in_any_range(id) {
                true => acc + 1,
                false => acc,
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
impl Cafeteria {
    fn is_id_in_any_range(&self, id: &usize) -> bool {
        self.merged
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
    }
}

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
