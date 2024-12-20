/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use crate::{Fro, Solution, TaskResult};
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Operation {
    Add,
    Multiply,
    Conc,
}

// Can add more shared vars here
pub struct BridgeRepair {
    data: Vec<(usize, Vec<usize>)>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for BridgeRepair {
    fn fro(data: &str) -> Self {
        Self {
            data: data
                .split('\n')
                .map(|line| {
                    if let Some((a, b)) = line.split_once(':') {
                        let a = a.parse::<usize>().unwrap();
                        let b = b
                            .split_ascii_whitespace()
                            .map(|n| n.trim().parse::<usize>().unwrap())
                            .collect();
                        (a, b)
                    } else {
                        panic!("invalid input data");
                    }
                })
                .collect(),
        }
    }
}

// Main solvers
impl Solution for BridgeRepair {
    fn silver(&self) -> TaskResult {
        let op = Vec::from([Operation::Add, Operation::Multiply]);
        TaskResult::Usize(
            self.data
                .iter()
                .map(|row| {
                    if Self::solve(&row.0, &row.1, &op) {
                        row.0
                    } else {
                        0
                    }
                })
                .sum(),
        )
    }

    fn gold(&self) -> TaskResult {
        let op = Vec::from([Operation::Add, Operation::Multiply, Operation::Conc]);
        TaskResult::Usize(
            self.data
                .par_iter()
                .map(|row| {
                    if Self::solve_recursive(row.0, &row.1, &op) {
                        row.0
                    } else {
                        0
                    }
                })
                .sum(),
        )
    }
}

// For assisting functions
impl BridgeRepair {
    fn solve(res: &usize, nums: &[usize], ops: &[Operation]) -> bool {
        // Use itertools to calculate all variations of operators
        (0..nums.len() - 1)
            .map(|_| ops.iter())
            .multi_cartesian_product()
            .any(|comb| {
                let mut sum = nums[0];
                for (i, c) in comb.iter().enumerate() {
                    match c {
                        Operation::Add => sum += nums[i + 1],
                        Operation::Multiply => sum *= nums[i + 1],
                        Operation::Conc => sum = Self::math_conc(&sum, &nums[i + 1]),
                    }
                    if sum > *res {
                        return false;
                    }
                }
                sum == *res
            })
    }

    fn solve_recursive(res: usize, nums: &[usize], ops: &[Operation]) -> bool {
        if nums.is_empty() {
            return false;
        }
        Self::try_operations(res, nums, ops, 1, nums[0])
    }

    // Recursive helper function.
    fn try_operations(
        res: usize,
        nums: &[usize],
        ops: &[Operation],
        idx: usize,
        current_sum: usize,
    ) -> bool {
        if idx == nums.len() {
            return current_sum == res;
        } // End of index
        if current_sum > res {
            return false;
        } // No solution found here

        for op in ops {
            let new_sum = match op {
                Operation::Add => current_sum + nums[idx],
                Operation::Multiply => current_sum * nums[idx],
                Operation::Conc => Self::math_conc(&current_sum, &nums[idx]),
            };

            if new_sum > res {
                continue;
            } // Early pruning
            if Self::try_operations(res, nums, ops, idx + 1, new_sum) {
                return true;
            }
        }
        false
    }

    fn _str_conc(a: &usize, b: &usize) -> usize {
        format!("{}{}", a, b).parse::<usize>().unwrap()
    }

    fn math_conc(a: &usize, b: &usize) -> usize {
        let mut b_temp = *b;
        let mut multiplier = 1;

        while b_temp > 0 {
            b_temp /= 10;
            multiplier *= 10;
        }

        a * multiplier + b
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/test/07.txt");
        let queue = BridgeRepair::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(3749));
        assert_eq!(queue.gold(), TaskResult::Usize(11387));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/real/07.txt");
        let queue = BridgeRepair::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(663613490587));
        assert_eq!(queue.gold(), TaskResult::Usize(110365987435001));
    }
}
