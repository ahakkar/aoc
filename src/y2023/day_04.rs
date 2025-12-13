/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use crate::{Fro, Solution, TaskResult};

// Can add more shared vars here
pub struct Scratchcards {
    data: Vec<String>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for Scratchcards {
    fn fro(input: &str) -> Self {
        Self {
            data: input.split('\n').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for Scratchcards {
    fn silver(&self) -> TaskResult {
        TaskResult::String("plii".to_string())
    }

    fn gold(&self) -> TaskResult {
        // Store counts of each card
        let mut counts = vec![1; self.data.len()];

        for (i, row_str) in self.data.iter().enumerate() {
            for n in 0..self.process_score(row_str) {
                counts[i + n + 1] += counts[i];
            }
        }

        // Sum the count of all cards
        counts.iter().sum::<usize>().into()
    }
}

// For assisting functions
impl Scratchcards {
    // count how many wins each game has
    fn process_score(&self, row_str: &str) -> usize {
        // discard all before ': ' in rows
        let (winning_nums, lottery_nums) = row_str
            .split_once(": ")
            .unwrap()
            .1
            .split_once(" | ")
            .unwrap();

        let winning_nums = winning_nums
            .split(' ')
            .filter_map(|n| n.parse::<usize>().ok())
            .collect::<std::collections::HashSet<_>>();

        // Count and return the matching numbers between winning & lottery
        lottery_nums
            .split(' ')
            .filter_map(|n| n.parse::<usize>().ok())
            .filter_map(|n| winning_nums.contains(&n).then_some(1))
            .sum()
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2023/test/04.txt");
        let queue = Scratchcards::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(13));
        assert_eq!(queue.gold(), TaskResult::Usize(30));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2023/real/4.txt");
        let queue = Scratchcards::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(24733));
        assert_eq!(queue.gold(), TaskResult::Usize(5422730));
    }
}
