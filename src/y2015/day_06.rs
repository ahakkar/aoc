/*
 * 2015 Advent of Code with Rust
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

use regex::Regex;

use crate::{Fro, Solution, TaskResult};

// Can add more shared vars here
pub struct ProbablyaFireHazard {
    data: Vec<String>,
}

const HEIGHT: usize = 1000;
const WIDTH: usize = 1000;
const OFF: u8 = 0;
const ON: u8 = 1;

// Can be used to implement fancier task-specific parsing
impl Fro for ProbablyaFireHazard {
    fn fro(input: &str) -> Self {
        Self {
            data: input.split('\n').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for ProbablyaFireHazard {
    fn silver(&self) -> TaskResult {
        let regex = Regex::new(r"^([^\d]*)(\d+),(\d+)[^\d]+(\d+),(\d+)").unwrap();
        let mut grid = vec![0u8; HEIGHT * WIDTH];
        //let mut ops = 0;

        for line in &self.data {
            if let Some(caps) = regex.captures(line) {
                let command = caps.get(1).unwrap().as_str().trim();

                let x1 = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
                let y1 = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
                let x2 = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
                let y2 = caps.get(5).unwrap().as_str().parse::<usize>().unwrap();

                if x1 > WIDTH - 1 || y1 > HEIGHT - 1 || x2 > WIDTH - 1 || y2 > HEIGHT - 1
                {
                    panic!("too high index");
                }

                //ops += (x2 - x1 + 1) * (y2 - y1 + 1);

                //println!("{} ({},{}) ({},{})", command, x1, y1, x2, y2);

                match command {
                    "turn on" => {
                        for row in y1..=y2 {
                            ProbablyaFireHazard::set_cells_in_range(
                                &mut grid, x1, x2, row, ON,
                            );
                        }
                    }
                    "turn off" => {
                        for row in y1..=y2 {
                            ProbablyaFireHazard::set_cells_in_range(
                                &mut grid, x1, x2, row, OFF,
                            );
                        }
                    }
                    "toggle" => {
                        for row in y1..=y2 {
                            let start = ProbablyaFireHazard::idx(x1, row, WIDTH);
                            let end = ProbablyaFireHazard::idx(x2 + 1, row, WIDTH);

                            for i in start..end {
                                grid[i] ^= 1;
                            }
                        }
                    }
                    _ => panic!("Invalid command"),
                }
            }
        }

        //println!("ops:{}", ops);
        TaskResult::Usize(grid.iter().map(|&b| b as usize).sum())
    }

    fn gold(&self) -> TaskResult {
        let regex = Regex::new(r"^([^\d]*)(\d+),(\d+)[^\d]+(\d+),(\d+)").unwrap();
        let mut grid = vec![0i8; HEIGHT * WIDTH];

        for line in &self.data {
            if let Some(caps) = regex.captures(line) {
                let command = caps.get(1).unwrap().as_str().trim();

                let x1 = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
                let y1 = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
                let x2 = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
                let y2 = caps.get(5).unwrap().as_str().parse::<usize>().unwrap();

                if x1 > WIDTH - 1 || y1 > HEIGHT - 1 || x2 > WIDTH - 1 || y2 > HEIGHT - 1
                {
                    panic!("too high index");
                }

                match command {
                    "turn on" => {
                        for row in y1..=y2 {
                            ProbablyaFireHazard::increment(&mut grid, x1, x2, row, 1);
                        }
                    }
                    "turn off" => {
                        for row in y1..=y2 {
                            ProbablyaFireHazard::increment(&mut grid, x1, x2, row, -1);
                        }
                    }
                    "toggle" => {
                        for row in y1..=y2 {
                            ProbablyaFireHazard::increment(&mut grid, x1, x2, row, 2);
                        }
                    }
                    _ => panic!("Invalid command"),
                }
            }
        }

        // 17325717 too low
        TaskResult::Usize(grid.iter().map(|&b| b as usize).sum())
    }
}

// For assisting functions
impl ProbablyaFireHazard {
    fn idx(x: usize, y: usize, width: usize) -> usize {
        y * width + x
    }

    fn set_cells_in_range(grid: &mut [u8], x1: usize, x2: usize, row: usize, value: u8) {
        let start = ProbablyaFireHazard::idx(x1, row, WIDTH);
        let end = ProbablyaFireHazard::idx(x2 + 1, row, WIDTH);

        grid[start..end].fill(value);
    }

    fn increment(grid: &mut [i8], x1: usize, x2: usize, row: usize, value: i8) {
        let start = ProbablyaFireHazard::idx(x1, row, WIDTH);
        let end = ProbablyaFireHazard::idx(x2 + 1, row, WIDTH);

        for col in start..end {
            // Clamp to 0
            grid[col] = (grid[col] + value).max(0);
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
        let test_data = read_data_from_file("input/2015/test/06.txt");
        let queue = ProbablyaFireHazard::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2015/real/06.txt");
        let queue = ProbablyaFireHazard::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(569999));
        assert_eq!(queue.gold(), TaskResult::Usize(17836115));
    }
}
