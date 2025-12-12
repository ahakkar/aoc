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

use rayon::vec;

use crate::{Fro, Solution, TaskResult, util::point::Point};

/*

0:
##.
.##
..#

1:
###
.##
##.

2:
#.#
###
#.#

3:
###
#..
###

4:
###
###
..#

5:
###
##.
#..

*/

// Can add more shared vars here
pub struct ChristmasTreeFarm {
    puzzles: Vec<(Point, Vec<i64>)>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for ChristmasTreeFarm {
    fn fro(input: &str) -> Self {
        let mut puzzles: Vec<(Point, Vec<i64>)> = vec![];
        for line in input
            .split('\n')
            .map(|line| line.to_string())
            .collect::<Vec<String>>()
        {
            let temp = line.split_once(':').unwrap();
            let size = temp
                .0
                .split_once('x')
                .map(|(a, b)| {
                    let x = a.parse::<i64>().unwrap();
                    let y = b.parse::<i64>().unwrap();
                    Point::new(x, y)
                })
                .unwrap();

            let parts = temp
                .1
                .split_ascii_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            puzzles.push((size, parts));
        }
        Self { puzzles }
    }
}

// Main solvers
impl Solution for ChristmasTreeFarm {
    // The puzzle is a "kompa", meaning there is ample space for all pieces in the
    // grids where the puzzles can even fit in the first place
    fn silver(&self) -> TaskResult {
        let mut valid = 0;
        for (i, puzzle) in self.puzzles.iter().enumerate() {
            println!("{} {:?}", puzzle.0, puzzle.1);

            let size = puzzle.0;
            let parts = &puzzle.1;

            if parts.iter().sum::<i64>() * 9 > size.x * size.y {
                continue;
            }

            valid += 1;
        }

        TaskResult::Usize(valid)
    }

    fn gold(&self) -> TaskResult {
        // No gold puzzle this time
        TaskResult::String("plaa".to_string())
    }
}

// For assisting functions
impl ChristmasTreeFarm {}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2025/test/12.txt");
        let queue = ChristmasTreeFarm::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2025/real/12.txt");
        let queue = ChristmasTreeFarm::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(591));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }
}
