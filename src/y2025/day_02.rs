/*
 * 2025 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use crate::{Fro, Solution, TaskResult};

// Can add more shared vars here
pub struct GiftShop {
    data: Vec<String>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for GiftShop {
    fn fro(input: &str) -> Self {
        Self {
            data: input.split(',').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for GiftShop {
    fn silver(&self) -> TaskResult {
        let mut silver: usize = 0;
        for item in &self.data {
            let (a, b) = item.split_once('-').expect("delimiter");
            let mut cur_str = a.to_string();
            let mut cur_int = a.parse::<usize>().unwrap();
            let end = b.parse::<usize>().unwrap();

            while cur_int <= end {
                if cur_str.len() % 2 == 0
                    && cur_str[0..cur_str.len() / 2] == cur_str[cur_str.len() / 2..]
                {
                    silver += cur_int;
                }

                cur_int += 1;
                cur_str = cur_int.to_string();
            }
        }
        TaskResult::Usize(silver)
    }

    fn gold(&self) -> TaskResult {
        let mut gold: usize = 0;
        for item in &self.data {
            let (a, b) = item.split_once('-').expect("delimiter");
            let mut cur_str = a.to_string();
            let mut cur_int = a.parse::<usize>().unwrap();
            let end = b.parse::<usize>().unwrap();

            while cur_int <= end {
                for substr_len in 1..=cur_str.len() / 2 {
                    if cur_str.len() % substr_len != 0 {
                        continue;
                    }
                    let pat = &cur_str[0..substr_len];
                    if cur_str == pat.repeat(cur_str.len() / substr_len) {
                        gold += cur_int;
                        break;
                    }
                }

                cur_int += 1;
                cur_str = cur_int.to_string();
            }
        }
        TaskResult::Usize(gold)
    }
}

// For assisting functions
impl GiftShop {}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2025/test/02.txt");
        let queue = GiftShop::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(1227775554));
        assert_eq!(queue.gold(), TaskResult::Usize(4174379265));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2025/real/02.txt");
        let queue = GiftShop::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(19219508902));
        assert_eq!(queue.gold(), TaskResult::Usize(27180728081));
    }
}
