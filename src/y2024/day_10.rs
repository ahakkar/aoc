/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use std::collections::HashSet;

use crate::{
    util::{
        self,
        utils::{Coord, Vec2D},
    },
    Fro, Solution, TaskResult,
};
use grid::*;
use util::utils::{EAST, NORTH, SOUTH, WEST};

// Can add more shared vars here
pub struct HoofIt {
    map: Grid<u32>,
    dirs: Vec<Vec2D>,
    starts: Vec<Coord>,
}

// Aggregators are used to abstract to what kind of save operation the
// end of recursion should do (save to a set, or increment an integer)
struct SetAggregator {
    ends: HashSet<Coord>,
}

struct CountAggregator {
    count: usize,
}

trait EndAggregator {
    fn add_end(&mut self, end: Coord);
    fn result(&self) -> usize;
}

impl SetAggregator {
    fn new() -> Self {
        Self {
            ends: HashSet::new(),
        }
    }
}

impl EndAggregator for SetAggregator {
    fn add_end(&mut self, end: Coord) {
        self.ends.insert(end);
    }

    fn result(&self) -> usize {
        self.ends.len()
    }
}

impl CountAggregator {
    fn new() -> Self {
        Self { count: 0 }
    }
}

impl EndAggregator for CountAggregator {
    fn add_end(&mut self, _end: Coord) {
        self.count += 1;
    }

    fn result(&self) -> usize {
        self.count
    }
}

// Can be used to implement fancier task-specific parsing
impl Fro for HoofIt {
    fn fro(input: &str) -> Self {
        let starts = input
            .lines()
            .enumerate()
            .flat_map(|(row, values)| {
                values.chars().enumerate().filter_map(move |(col, ch)| {
                    ch.to_digit(10).filter(|&digit| digit == 0).map(|_| Coord {
                        x: col as isize,
                        y: row as isize,
                    })
                })
            })
            .collect();

        let map: Grid<u32> = Grid::from_vec(
            input
                .replace('\n', "")
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect(),
            (input.len() as f64).sqrt() as usize,
        );

        let dirs = Vec::from([NORTH, SOUTH, EAST, WEST]);

        Self { map, dirs, starts }
    }
}

// Main solvers
impl Solution for HoofIt {
    fn silver(&self) -> TaskResult {
        TaskResult::Usize(
            self.starts
                .iter()
                .map(|start| {
                    let mut aggregator = SetAggregator::new();
                    self.seek_ends(start, &mut aggregator);
                    aggregator.result()
                })
                .sum(),
        )
    }

    fn gold(&self) -> TaskResult {
        TaskResult::Usize(
            self.starts
                .iter()
                .map(|start| {
                    let mut aggregator = CountAggregator::new();
                    self.seek_ends(start, &mut aggregator);
                    aggregator.result()
                })
                .sum(),
        )
    }
}

// For assisting functions
impl HoofIt {
    // Wrapper for Grid crate, it wants (row, col) = (y, x) for some reason
    fn get_value(&self, x: &isize, y: &isize) -> Option<&u32> {
        self.map.get(*y, *x)
    }

    fn seek_ends<A: EndAggregator>(&self, current: &Coord, aggregator: &mut A) {
        if *Self::get_value(self, &current.x, &current.y).unwrap() == 9 {
            aggregator.add_end(*current);
            return;
        }

        for dir in &self.dirs {
            let dx = current.x + dir.x;
            let dy = current.y + dir.y;

            if let Some(neighbor) = Self::get_value(self, &dx, &dy) {
                // Check for neighbours if their value is 1 higher
                if Self::get_value(self, &current.x, &current.y).unwrap() + 1 == *neighbor
                {
                    Self::seek_ends(self, &Coord { x: dx, y: dy }, aggregator);
                }
            }
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
        let test_data = read_data_from_file("input/test/10.txt");
        let queue = HoofIt::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(36));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/real/10.txt");
        let queue = HoofIt::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(822));
        assert_eq!(queue.gold(), TaskResult::Usize(1801));
    }
}
