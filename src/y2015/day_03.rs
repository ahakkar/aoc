/*
 * 2015 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use crate::util::point2::{EAST, NORTH, Point2, SOUTH, WEST};
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
        let mut houses: HashMap<Point2, usize> = HashMap::new();
        let mut current = Point2::new(0, 0);

        // Santa visits the starting house
        houses.insert(current, 1);

        for c in self.data.chars() {
            let next: Point2 = match c {
                '>' => current + EAST,
                '<' => current + WEST,
                '^' => current + NORTH,
                'v' => current + SOUTH,
                _ => panic!("unsupported char"),
            };

            current = next;
            *houses.entry(next).or_default() += 1;
        }

        TaskResult::Usize(houses.len())
    }

    fn gold(&self) -> TaskResult {
        // loc, visits
        let mut houses: HashMap<Point2, usize> = HashMap::new();
        let mut santa_robo = [Point2::new(0, 0), Point2::new(0, 0)];

        // Santa & robo both visit the starting house
        houses.insert(Point2::new(0, 0), 2);

        // Move santa or robo with even-odd commands
        for (i, c) in self.data.chars().enumerate() {
            let p = &mut santa_robo[i % 2];

            *p = match c {
                '>' => *p + EAST,
                '<' => *p + WEST,
                '^' => *p + NORTH,
                'v' => *p + SOUTH,
                _ => panic!("unsupported char"),
            };

            *houses.entry(*p).or_default() += 1;
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
