/*
 * 2015 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use crate::util::point::{EAST, NORTH, Point, SOUTH, WEST};
use crate::{Fro, Solution, TaskResult};
use std::collections::HashMap;

// Can add more shared vars here
pub struct PerfectlySphericalHousesinaVacuum {
    data: String,
}

// Can be used to implement fancier task-specific parsing
impl Fro for PerfectlySphericalHousesinaVacuum {
    fn fro(input: &str) -> Self {
        Self {
            data: input.to_string(),
        }
    }
}

// Main solvers
impl Solution for PerfectlySphericalHousesinaVacuum {
    fn silver(&self) -> TaskResult {
        // loc, visits
        let mut houses: HashMap<Point, usize> = HashMap::new();
        let mut current = Point::new(0, 0);
        houses.insert(current, 1);

        for c in self.data.chars() {
            let next: Point = match c {
                '>' => current + EAST,
                '<' => current + WEST,
                '^' => current + NORTH,
                'v' => current + SOUTH,
                _ => panic!("unsupported char"),
            };

            current = next;

            if let Some(visit) = houses.get_mut(&next) {
                *visit += 1;
            } else {
                houses.insert(next, 1);
            }
        }

        TaskResult::Usize(houses.len())
    }

    fn gold(&self) -> TaskResult {
        // loc, visits
        let mut houses: HashMap<Point, usize> = HashMap::new();
        let mut santa = Point::new(0, 0);
        let mut robo = Point::new(0, 0);
        houses.insert(Point::new(0, 0), 2);

        for (i, c) in self.data.chars().enumerate() {
            let mut current: Point;

            if i % 2 == 0 {
                current = santa;
            } else {
                current = robo;
            }

            let next: Point = match c {
                '>' => current + EAST,
                '<' => current + WEST,
                '^' => current + NORTH,
                'v' => current + SOUTH,
                _ => panic!("unsupported char"),
            };

            current = next;
            if i % 2 == 0 {
                santa = current;
            } else {
                robo = current;
            }

            if let Some(visit) = houses.get_mut(&next) {
                *visit += 1;
            } else {
                houses.insert(next, 1);
            }
        }

        TaskResult::Usize(houses.len())
    }
}

// For assisting functions
impl PerfectlySphericalHousesinaVacuum {}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2015/test/03.txt");
        let queue = PerfectlySphericalHousesinaVacuum::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2015/real/03.txt");
        let queue = PerfectlySphericalHousesinaVacuum::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }
}
