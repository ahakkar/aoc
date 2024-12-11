/*
 * 2024 Advent of Code with Rust
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

use std::{collections::HashMap, iter, os::unix::process};

use rayon::vec;

use super::utils::*;
use crate::{Fro, Solution, TaskResult};

// Can add more shared vars here
pub struct PlutonianPebbles {
    data: Vec<usize>,
    digit_count_count: usize,
}

// Can be used to implement fancier task-specific parsing
impl Fro for PlutonianPebbles {
    fn fro(input: &str) -> Self {
        Self {
            data: input
                .split_ascii_whitespace()
                .map(|c| c.parse::<usize>().unwrap())
                .collect(),
            digit_count_count: 0,
        }
    }
}

// Main solvers
impl Solution for PlutonianPebbles {
    fn silver(&self) -> TaskResult {
        TaskResult::Usize(Self::process_a(self, self.data.clone(), &mut 25))
    }

    fn gold(&self) -> TaskResult {
        let mut hash: HashMap<usize, usize> = HashMap::new();
        for val in &self.data {
            *hash.entry(*val).or_insert(1) = 1;
        }

        println!("Gold data: {:?}", hash);

        TaskResult::Usize(Self::process_b(self, &mut hash, &mut 75))
        //TaskResult::Usize(0)
    }
}

/*
 0 -> 1.

  even number of digits -> it is replaced by two stones.
  The left half of the digits are engraved on
  the new left stone, and the right half of the digits are engraved on the
  new right stone. (The new numbers don't keep extra leading zeroes: 1000
  would become stones 10 and 0.)

  else = old * 2024
*/

// For assisting functions
impl PlutonianPebbles {
    fn process_a(&self, nums: Vec<usize>, iter_left: &mut usize) -> usize {
        if *iter_left == 0 {
            return nums.len();
        }

        let mut process: Vec<usize> = Vec::with_capacity(nums.len() * 2);

        for val in nums {
            if val == 0 {
                process.push(1);
            } else if Self::digit_count(val) % 2 == 0 {
                let (l, r) = Self::split_number(self, val);
                process.push(l);
                process.push(r);
            } else {
                process.push(val * 2024);
            }
        }

        *iter_left -= 1;
        return Self::process_a(self, process, iter_left);
    }

    fn process_b(
        &self,
        nums: &mut HashMap<usize, usize>,
        iter_left: &mut usize,
    ) -> usize {
        if *iter_left == 0 {
            return nums.values().sum();
        }

        // Can't modify a map in place
        let mut changes: Vec<(usize, isize)> = vec![];
        for (key, val) in nums.iter() {
            let change = *val as isize;
            if *key == 0 {
                changes.push((1, change));
                changes.push((0, -change));
            } else if Self::digit_count(*key) % 2 == 0 {
                let (l, r) = self.split_number(*key);
                changes.push((l, change));
                changes.push((r, change));
                changes.push((*key, -change));
            } else {
                changes.push((*key * 2024, change));
                changes.push((*key, -change));
            }
        }

        // Apply changes after iteration
        for (k, delta) in changes {
            if delta > 0 {
                *nums.entry(k).or_insert(0) += delta as usize;
            } else if let Some(count) = nums.get_mut(&k) {
                *count = count.saturating_sub((-delta) as usize);
                if *count == 0 {
                    nums.remove(&k);
                }
            }
        }

        *iter_left -= 1;
        return Self::process_b(self, nums, iter_left);
    }

    fn digit_count(n: usize) -> u32 {
        if n < 10 {
            1
        } else if n < 100 {
            2
        } else if n < 1_000 {
            3
        } else if n < 10_000 {
            4
        } else if n < 100_000 {
            5
        } else if n < 1_000_000 {
            6
        } else if n < 10_000_000 {
            7
        } else if n < 100_000_000 {
            8
        } else if n < 1_000_000_000 {
            9
        } else if n < 10_000_000_000 {
            10
        } else if n < 100_000_000_000 {
            11
        } else if n < 1_000_000_000_000 {
            12
        } else if n < 10_000_000_000_000 {
            13
        } else if n < 100_000_000_000_000 {
            14
        } else if n < 1_000_000_000_000_000 {
            15
        } else if n < 10_000_000_000_000_000 {
            16
        } else if n < 100_000_000_000_000_000 {
            17
        } else if n < 1_000_000_000_000_000_000 {
            18
        } else if n < 10_000_000_000_000_000_000 {
            19
        } else {
            20
        }
    }

    fn split_number(&self, n: usize) -> (usize, usize) {
        let divisor = 10usize.pow(Self::digit_count(n) / 2);
        (n / divisor, n % divisor)
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/test/11.txt");
        let queue = PlutonianPebbles::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(55312));
        assert_eq!(queue.gold(), TaskResult::Usize(55312));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/real/11.txt");
        let queue = PlutonianPebbles::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(189167));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }
}
