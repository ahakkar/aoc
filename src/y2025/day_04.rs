/*
 * 2025 Advent of Code with Rust
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

use crate::{
    Fro, Solution, TaskResult,
    util::{
        self,
        point::{EAST, NORTH, Point, SOUTH, WEST},
        utils::Direction,
    },
};
use grid::Grid;
use num_integer::sqrt;
use util::grid::XyGrid;

// Can add more shared vars here
pub struct PrintingDepartment {
    map: Grid<char>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for PrintingDepartment {
    fn fro(input: &str) -> Self {
        let data: Vec<char> = input.replace("\n", "").chars().collect();
        Self {
            map: Grid::from_vec(
                input.replace("\n", "").chars().collect(),
                sqrt(data.len()),
            ),
        }
    }
}

// Main solvers
impl Solution for PrintingDepartment {
    fn silver(&self) -> TaskResult {
        let mut silver = 0;

        for row in 0..self.map.rows() {
            for col in 0..self.map.cols() {
                // Ignore non-paper cells
                if let Some('.') = self.map.get(row, col) {
                    continue;
                }
                // Check if a paper cell has at most 3 paper neighbors
                if let Some(n) = self.map.get_neighbors(col, row) {
                    if n.iter().filter(|c| ***c == '@').count() < 4 {
                        silver += 1;
                    }
                }
            }
        }

        TaskResult::Usize(silver)
    }

    fn gold(&self) -> TaskResult {
        let mut gold = 0;
        let mut copy = self.map.clone();
        let mut changed = true;

        // Loop until no further rolls can be removed
        while changed {
            changed = false;
            for row in 0..self.map.rows() {
                for col in 0..self.map.cols() {
                    // Ignore non-paper cells
                    if let Some('.') = copy.get(row, col) {
                        continue;
                    }
                    // Check if a paper cell has at most 3 paper neighbors
                    if let Some(n) = copy.get_neighbors(col, row) {
                        if n.iter().filter(|c| ***c == '@').count() < 4 {
                            // Mark roll as removed by forklifts
                            *copy.get_mut(row, col).unwrap() = '.';
                            gold += 1;
                            changed = true;
                        }
                    }
                }
            }
        }

        TaskResult::Usize(gold)
    }
}

// For assisting functions
impl PrintingDepartment {}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2025/test/04.txt");
        let queue = PrintingDepartment::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(13));
        assert_eq!(queue.gold(), TaskResult::Usize(43));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2025/real/04.txt");
        let queue = PrintingDepartment::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(1457));
        assert_eq!(queue.gold(), TaskResult::Usize(8310));
    }
}
