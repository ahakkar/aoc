/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use crate::{Fro, Solution, TaskResult};

// Can add more shared vars here
pub struct WaitForIt {
    data: Vec<String>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for WaitForIt {
    fn fro(input: &str) -> Self {
        Self {
            data: input.split('\n').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for WaitForIt {
    fn silver(&self) -> TaskResult {
        (calc_dist(62, 553)
            * calc_dist(64, 1010)
            * calc_dist(91, 1473)
            * calc_dist(90, 1074))
        .into()
    }

    fn gold(&self) -> TaskResult {
        (calc_dist(62649190, 553101014731074) as usize).into()
    }
}

// For assisting functions
impl WaitForIt {
    fn calc_dist(time: i64, limit: i64) -> i64 {
        let s = ((time * time - 4 * limit) as f64).sqrt();
        let mut x0 = ((time as f64) - s) / 2.0;
        let mut x1 = ((time as f64) + s) / 2.0;

        // if for example x0 = 10 and x10= 20 (integer results)
        if x0 == x0.ceil() {
            x0 += 1.0;
        } else {
            x0 = x0.ceil();
        }
        if x1 == x1.floor() {
            x1 -= 1.0
        } else {
            x1 = x1.floor();
        }

        (x1 - x0 + 1.0) as i64
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2023/test/06.txt");
        let queue = WaitForIt::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2023/real/06.txt");
        let queue = WaitForIt::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(840336));
        assert_eq!(queue.gold(), TaskResult::Usize(41382569));
    }
}
