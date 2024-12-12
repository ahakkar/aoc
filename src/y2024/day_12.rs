/*
 * 2024 Advent of Code with Rust
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

use crate::{
    util::{self, point},
    Fro, Solution, TaskResult,
};
use grid::*;
use util::point::{EAST, NORTH, SOUTH, WEST};

// Can add more shared vars here
pub struct GardenGroups {
    map: Grid<Plot>,
}

struct Plot {
    ptype: char,
    visited: bool,
}

impl Plot {
    pub const fn new(ptype: char, visited: bool) -> Plot {
        Plot { ptype, visited }
    }
}

// Can be used to implement fancier task-specific parsing
impl Fro for GardenGroups {
    fn fro(input: &str) -> Self {
        Self {
            map: Grid::from_vec(
                input
                    .replace('\n', "")
                    .chars()
                    .map(|c| Plot::new(c, false))
                    .collect(),
                (input.len() as f64).sqrt() as usize,
            ),
        }
    }
}

// Main solvers
impl Solution for GardenGroups {
    fn silver(&self) -> TaskResult {
        /*   Noh kokeilen flood fillata ei-visited tilet

        ja annan joka regionille juoksevan id:n ja tallennan sen esim vektoriin jossa on sen indeksin takana perimeterin pituus.

        Sen voi laskea flood fillatessa sivuista jotka kuuluu eri regioniin tai kartan reunaan */

        TaskResult::String("plii".to_string())
    }

    fn gold(&self) -> TaskResult {
        TaskResult::String("plaa".to_string())
    }
}

// For assisting functions
impl GardenGroups {}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/test/12.txt");
        let queue = GardenGroups::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/real/12.txt");
        let queue = GardenGroups::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }
}
