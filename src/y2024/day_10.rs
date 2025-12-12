/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use std::collections::HashSet;

use crate::{
    Fro, Solution, TaskResult,
    util::{direction::Direction, point2::Point2},
};
use grid::*;

// Can add more shared vars here
pub struct HoofIt {
    map: Grid<u32>,
    starts: Vec<Point2>,
}

// Aggregators are used to abstract to what kind of save operation the
// end of recursion should do (save to a set, or increment an integer)
struct SetAggregator {
    ends: HashSet<Point2>,
}

struct CountAggregator {
    count: usize,
}

trait EndAggregator {
    fn add_end(&mut self, end: Point2);
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
    fn add_end(&mut self, end: Point2) {
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
    fn add_end(&mut self, _end: Point2) {
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
                    ch.to_digit(10).filter(|&digit| digit == 0).map(|_| Point2 {
                        x: col as i64,
                        y: row as i64,
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

        Self { map, starts }
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
    fn get_value(&self, point: &Point2) -> Option<&u32> {
        self.map.get(point.y, point.x)
    }

    fn seek_ends<A: EndAggregator>(&self, current: &Point2, aggregator: &mut A) {
        if *Self::get_value(self, current).unwrap() == 9 {
            aggregator.add_end(*current);
            return;
        }

        for dir in Direction::ORTHOGONAL {
            let next = current.step(dir);

            if let Some(neighbor) = Self::get_value(self, &next) {
                // Check for neighbours if their value is 1 higher
                if Self::get_value(self, current).unwrap() + 1 == *neighbor {
                    Self::seek_ends(self, &next, aggregator);
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
        let test_data = read_data_from_file("input/2024/test/10.txt");
        let queue = HoofIt::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(36));
        assert_eq!(queue.gold(), TaskResult::Usize(81));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2024/real/10.txt");
        let queue = HoofIt::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(822));
        assert_eq!(queue.gold(), TaskResult::Usize(1801));
    }
}
