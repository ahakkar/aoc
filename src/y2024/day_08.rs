/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use crate::{Fro, Solution, TaskResult};
use itertools::Itertools;
use std::collections::HashMap;

// Can add more shared vars here
pub struct ResonantCollinearity {
    points: HashMap<char, Vec<[usize; 2]>>,
    rows: usize,
    cols: usize,
}

// Can be used to implement fancier task-specific parsing
impl Fro for ResonantCollinearity {
    // Parse data to list of Points
    fn fro(data: &str) -> Self {
        let mut points: HashMap<char, Vec<[usize; 2]>> = HashMap::new();
        let mut rows = 0;
        let mut cols = 0;

        for (row_idx, row) in data.lines().enumerate() {
            rows = row_idx + 1;
            cols = cols.max(row.chars().count());
            for (col_idx, ch) in row.chars().enumerate() {
                if ch != '.' {
                    points.entry(ch).or_default().push([col_idx, row_idx]);
                }
            }
        }

        Self { points, rows, cols }
    }
}

// Main solvers
impl Solution for ResonantCollinearity {
    fn silver(&self) -> TaskResult {
        // 2D bool vec is faster than hashset
        let mut set = vec![vec![false; self.rows]; self.cols];

        for point_vec in self.points.values() {
            point_vec.iter().combinations(2).for_each(|pair| {
                let p3 = Self::point_at_line(pair[0], pair[1], 1);
                if Self::fits_bounds(self, &p3) {
                    set[p3[0]][p3[1]] = true
                }
                let p4 = Self::point_at_line(pair[1], pair[0], 1);
                if Self::fits_bounds(self, &p4) {
                    set[p4[0]][p4[1]] = true
                }
            });
        }
        TaskResult::Usize(Self::set_size(set))
    }

    fn gold(&self) -> TaskResult {
        let mut set = vec![vec![false; self.rows]; self.cols];

        for point_vec in self.points.values() {
            point_vec.iter().combinations(2).for_each(|pair| {
                Self::points_on_line(self, pair).iter().for_each(|point| {
                    set[point[0]][point[1]] = true;
                })
            });
        }
        TaskResult::Usize(Self::set_size(set))
    }
}

// For assisting functions
impl ResonantCollinearity {
    fn set_size(set: Vec<Vec<bool>>) -> usize {
        set.iter().flat_map(|r| r.iter()).filter(|&&v| v).count()
    }

    fn fits_bounds(&self, p: &[usize; 2]) -> bool {
        p[0] < self.cols && p[1] < self.rows
    }

    fn point_at_line(p1: &[usize; 2], p2: &[usize; 2], m: usize) -> [usize; 2] {
        let v = [p2[0] - p1[0], p2[1] - p1[1]];
        [p1[0] - m * v[0], p1[1] - m * v[1]]
    }

    fn points_on_line(&self, pair: Vec<&[usize; 2]>) -> Vec<[usize; 2]> {
        std::iter::once(*pair[0])
            .chain(std::iter::once(*pair[1]))
            .chain(self.extend_line(pair[0], pair[1]))
            .chain(self.extend_line(pair[1], pair[0]))
            .collect()
    }

    fn extend_line(&self, start: &[usize; 2], end: &[usize; 2]) -> Vec<[usize; 2]> {
        (1..)
            .map(|m| Self::point_at_line(start, end, m))
            .take_while(|np| Self::fits_bounds(self, np))
            .collect()
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2024/test/08.txt");
        let queue = ResonantCollinearity::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(14));
        assert_eq!(queue.gold(), TaskResult::Usize(34));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2024/real/08.txt");
        let queue = ResonantCollinearity::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(318));
        assert_eq!(queue.gold(), TaskResult::Usize(1126));
    }
}
