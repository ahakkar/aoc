/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

#![allow(clippy::needless_range_loop)]

use crate::{Fro, Solution, TaskResult};
use regex::Regex;

// Can add more shared vars here
pub struct GearRatios {
    data: Vec<String>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for GearRatios {
    fn fro(input: &str) -> Self {
        Self {
            data: input.split('\n').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for GearRatios {
    fn silver(&self) -> TaskResult {
        let re_id: regex::Regex = Regex::new(r"\d{1,3}").unwrap();

        let mut sum = 0;

        let mut twod_grid: Vec<Vec<char>> = vec![];

        self.conv_to_2d_grid(&self.data, &mut twod_grid);

        for (y, row) in self.data.iter().enumerate() {
            let ids_iter = re_id.find_iter(row);

            // Add the id to sum if it has a symbol as neighbour
            for iter in ids_iter {
                if self.check_neighbours_silver(&twod_grid, &y, &iter) {
                    sum += iter.as_str().parse::<usize>().unwrap();
                }
            }
        }
        sum.into()
    }

    fn gold(&self) -> TaskResult {
        let re_symbol: regex::Regex = Regex::new(r"[*]").unwrap();

        let mut sum = 0;
        for (row, row_str) in self.data.iter().enumerate() {
            for gear in re_symbol.find_iter(row_str) {
                sum += self.check_neighbours_gold(&self.data, &row, &gear.start());
            }
        }
        (sum as usize).into()
    }
}

// For assisting functions
impl GearRatios {
    fn check_neighbours_silver(
        &self,
        twod_grid: &[Vec<char>],
        row: &usize,
        id_iter: &regex::Match<'_>,
    ) -> bool {
        let re_symbols: regex::Regex = Regex::new(r"[-*#&+$%/@=]").unwrap();
        let mut neighbours: String = String::new();
        let grid_height = twod_grid.len();
        let grid_width = twod_grid[0].len();

        let start = id_iter.start();
        let end = id_iter.end();
        //let length = end - start;

        let row_start = if row > &0 { row - 1 } else { 0 };
        let row_end = std::cmp::min(row + 2, grid_height);
        let col_start = if start > 0 { start - 1 } else { 0 };
        let col_end = std::cmp::min(end + 1, grid_width);

        // Iterate over the surrounding cells
        for r in row_start..row_end {
            for c in col_start..col_end {
                // Skip the actual numbers
                if r == *row && (c >= start && c < end) {
                    continue;
                }
                if let Some(ch) = twod_grid[r].get(c) {
                    neighbours.push(*ch);
                }
            }
        }

        re_symbols.find(neighbours.as_str()).is_some()
    }

    /**
     *   0  1  2  3  4  5  6  7
     *   8  9 10  * 12 13 14 15
     *  16 17 18 19 20 21 22 23
     */
    fn check_neighbours_gold(
        &self,
        twod_grid: &[String],
        row: &usize,
        col: &usize,
    ) -> i64 {
        let re_id: regex::Regex = Regex::new(r"\d{1,3}").unwrap();
        let start = col.saturating_sub(3);
        let end = start + 7;

        let mut neighbours: String = String::new();
        let mut n1: i64 = 0;
        let mut n2: i64 = 0;

        // iterate through rows -1, 0, 1
        for i in 0..3 {
            neighbours += &twod_grid[row + i - 1][start..end];
            neighbours.push('|');
        }

        // check if any numbers are next to the gear in the flattened view
        for num in re_id.find_iter(&neighbours) {
            if (num.start()..num.end())
                .any(|index| [2, 3, 4, 10, 12, 18, 19, 20].contains(&index))
            {
                let num_value = num.as_str().parse::<i64>().unwrap_or(-1);
                match (n1, n2) {
                    (0, _) => n1 = num_value,
                    (_, 0) => n2 = num_value,
                    _ => (),
                }
            }
        }

        n1 * n2
    }

    fn conv_to_2d_grid(&self, data: &[String], twod_grid: &mut Vec<Vec<char>>) {
        for row in data {
            let mut twod_row: Vec<char> = vec![];
            for char in row.chars() {
                twod_row.push(char);
            }
            twod_grid.push(twod_row);
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
        let test_data = read_data_from_file("input/2023/test/03.txt");
        let queue = GearRatios::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(4361));
        assert_eq!(queue.gold(), TaskResult::Usize(467835));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2023/real/03.txt");
        let queue = GearRatios::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(530849));
        assert_eq!(queue.gold(), TaskResult::Usize(84900879));
    }
}
