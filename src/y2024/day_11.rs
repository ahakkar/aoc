/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use crate::{Fro, Solution, TaskResult};
use std::collections::HashMap;

// Can add more shared vars here
pub struct PlutonianPebbles {
    data: Vec<usize>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for PlutonianPebbles {
    fn fro(input: &str) -> Self {
        Self {
            data: input
                .split_ascii_whitespace()
                .map(|c| c.parse::<usize>().unwrap())
                .collect(),
        }
    }
}

// Main solvers
impl Solution for PlutonianPebbles {
    fn silver(&self) -> TaskResult {
        let mut hash: HashMap<usize, usize> = HashMap::new();
        for val in &self.data {
            *hash.entry(*val).or_insert(1) = 1;
        }
        TaskResult::Usize(Self::process(self, &mut hash, &mut 25))
    }

    fn gold(&self) -> TaskResult {
        let mut hash: HashMap<usize, usize> = HashMap::new();
        for val in &self.data {
            *hash.entry(*val).or_insert(1) = 1;
        }
        TaskResult::Usize(Self::process(self, &mut hash, &mut 75))
    }
}

// For assisting functions
impl PlutonianPebbles {
    fn process(&self, nums: &mut HashMap<usize, usize>, iter_left: &mut usize) -> usize {
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
            } else if Self::count_digits(*key as u64) % 2 == 0 {
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
        Self::process(self, nums, iter_left)
    }

    fn _count_digits_str(n: usize) -> usize {
        n.to_string().len()
    }

    fn _count_digits_match(n: usize) -> u32 {
        match n {
            0..=9 => 1,
            10..=99 => 2,
            100..=999 => 3,
            1_000..=9_999 => 4,
            10_000..=99_999 => 5,
            100_000..=999_999 => 6,
            1_000_000..=9_999_999 => 7,
            10_000_000..=99_999_999 => 8,
            100_000_000..=999_999_999 => 9,
            1_000_000_000..=9_999_999_999 => 10,
            10_000_000_000..=99_999_999_999 => 11,
            100_000_000_000..=999_999_999_999 => 12,
            1_000_000_000_000..=9_999_999_999_999 => 13,
            10_000_000_000_000..=99_999_999_999_999 => 14,
            100_000_000_000_000..=999_999_999_999_999 => 15,
            1_000_000_000_000_000..=9_999_999_999_999_999 => 16,
            10_000_000_000_000_000..=99_999_999_999_999_999 => 17,
            100_000_000_000_000_000..=999_999_999_999_999_999 => 18,
            1_000_000_000_000_000_000..=9_999_999_999_999_999_999 => 19,
            _ => panic!("Too big number"),
        }
    }

    #[rustfmt::skip]
    fn _count_digits_ifelse(n: usize) -> u32 {
        if n < 10 { 1 }
        else if n < 100 { 2 }
        else if n < 1_000 { 3 }
        else if n < 10_000 { 4 }
        else if n < 100_000 { 5 }
        else if n < 1_000_000 { 6 }
        else if n < 10_000_000 { 7 }
        else if n < 100_000_000 { 8 }
        else if n < 1_000_000_000 { 9 }
        else if n < 10_000_000_000 { 10 }
        else if n < 100_000_000_000 { 11 }
        else if n < 1_000_000_000_000 { 12 }
        else if n < 10_000_000_000_000 { 13 }
        else if n < 100_000_000_000_000 { 14 }
        else if n < 1_000_000_000_000_000 { 15 }
        else if n < 10_000_000_000_000_000 { 16 }
        else if n < 100_000_000_000_000_000 { 17 }
        else if n < 1_000_000_000_000_000_000 { 18 }
        else if n < 10_000_000_000_000_000_000 { 19 }
        else { panic!("Too long number") }           
    }

    #[rustfmt::skip]
    fn count_digits(n: u64) -> u32 {
        const POW10: [u64; 20] = [
                      /*
                      ***
                       */
                       0,
                       10,
                      100,
                     1_000,
                     10_000,
                    100_000,
                    1_000_000,
                   10_000_000,
                   100_000_000,
                  1_000_000_000,
                 10_000_000_000,
                 100_000_000_000,
                1_000_000_000_000,
               10_000_000_000_000,
               100_000_000_000_000,
              1_000_000_000_000_000,
              10_000_000_000_000_000,
             100_000_000_000_000_000,
            1_000_000_000_000_000_000,
            10_000_000_000_000_000_000,
        ];

        let t = ((64 - (n | 1).leading_zeros()) * 1233) >> 12;
        t - if n < POW10[t as usize] { 1 } else { 0 } + 1
    }

    fn split_number(&self, n: usize) -> (usize, usize) {
        let divisor = 10usize.pow(Self::count_digits(n as u64) / 2);
        (n / divisor, n % divisor)
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/test/11.txt");
        let queue = PlutonianPebbles::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(55312));
        assert_eq!(queue.gold(), TaskResult::Usize(65601038650482));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/real/11.txt");
        let queue = PlutonianPebbles::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(189167));
        assert_eq!(queue.gold(), TaskResult::Usize(225253278506288));
    }
}
