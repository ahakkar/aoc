/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

use crate::{Fro, Solution, TaskResult};

// Can add more shared vars here
pub struct MirageMaintenance {
    data: Vec<String>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for MirageMaintenance {
    fn fro(input: &str) -> Self {
        Self {
            data: input.split('\n').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for MirageMaintenance {
    fn silver(&self) -> TaskResult {
        let mut sum: i64 = 0;

        for row in &self.data {
            let mut cum: i64 = 0;
            let mut series: Vec<i64> = vec![];
            let mut nums: Vec<i64> = row
                .split_ascii_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();

            series.push(*nums.last().unwrap());

            while nums.iter().sum::<i64>() != 0 {
                let mut new_nums: Vec<i64> = vec![];
                for i in 0..nums.len() - 1 {
                    new_nums.push(nums[i + 1] - nums[i]);
                }
                series.push(*new_nums.last().unwrap());
                nums = new_nums;
            }

            for i in (0..series.len()).rev() {
                cum += series[i];
            }
            sum += cum;
        }
        (sum as usize).into()
    }

    fn gold(&self) -> TaskResult {
        let mut sum: i64 = 0;

        for row in &self.data {
            let mut prev: i64 = 0;
            let mut series: Vec<Vec<i64>> = vec![];
            let mut nums: Vec<i64> = row
                .split_ascii_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();

            series.push((*nums).to_vec());
            while nums.iter().sum::<i64>() != 0 {
                let mut new_nums: Vec<i64> = vec![];
                for i in 0..nums.len() - 1 {
                    new_nums.push(nums[i + 1] - nums[i]);
                }
                series.push((*new_nums).to_vec());
                nums = new_nums;
            }

            //println!("series: {:?}\n", series);

            //print!("cum: ");
            for i in (0..series.len()).rev() {
                let b = series[i].first().unwrap();
                //println!("{} - {} = {}", b, prev, b - prev);
                prev = b - prev;
            }
            sum += prev;
        }
        (sum as usize).into()
    }
}

// For assisting functions
impl MirageMaintenance {}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2023/test/09.txt");
        let queue = MirageMaintenance::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2023/real/09.txt");
        let queue = MirageMaintenance::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }
}
