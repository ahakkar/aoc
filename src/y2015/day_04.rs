/*
 * 2015 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use crate::{Fro, Solution, TaskResult};

// Can add more shared vars here
pub struct TheIdealStockingStuffer {
    data: String,
}

// Can be used to implement fancier task-specific parsing
impl Fro for TheIdealStockingStuffer {
    fn fro(input: &str) -> Self {
        Self {
            data: input.trim().to_string(),
        }
    }
}

// Main solvers
impl Solution for TheIdealStockingStuffer {
    fn silver(&self) -> TaskResult {
        TheIdealStockingStuffer::starts_with_n_zeroes(&self.data, 5).into()
    }

    fn gold(&self) -> TaskResult {
        TheIdealStockingStuffer::starts_with_n_zeroes(&self.data, 6).into()
    }
}

// For assisting functions
impl TheIdealStockingStuffer {
    fn starts_with_n_zeroes(prefix: &str, n: usize) -> usize {
        let target = "0".repeat(n);
        let mut num = 1;

        loop {
            let test = format!("{}{}", prefix, num);
            let hash = format!("{:x}", md5::compute(test.as_bytes()));

            if hash.starts_with(&target) {
                return num;
            }
            num += 1;
        }
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2015/test/04.txt");
        let queue = TheIdealStockingStuffer::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(117946));
        assert_eq!(queue.gold(), TaskResult::Usize(3938038));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2015/real/04.txt");
        let queue = TheIdealStockingStuffer::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(117946));
        assert_eq!(queue.gold(), TaskResult::Usize(3938038));
    }
}
