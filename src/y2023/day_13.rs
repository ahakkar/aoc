/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

use crate::{Fro, Solution, TaskResult};
use std::cmp::min;

const OFFSET: isize = 1;

// Can add more shared vars here
pub struct PointofIncidence {
    data: Vec<String>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for PointofIncidence {
    fn fro(input: &str) -> Self {
        Self {
            data: input.split('\n').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for PointofIncidence {
    fn silver(&self) -> TaskResult {
        let result =
            PointofIncidence::process_data(&self.data, PointofIncidence::process_silver);

        TaskResult::Usize(result as usize)
    }

    fn gold(&self) -> TaskResult {
        let result =
            PointofIncidence::process_data(&self.data, PointofIncidence::process_gold);
        TaskResult::Usize(result as usize)
    }
}

// For assisting functions
impl PointofIncidence {
    fn process_silver(map: &[String]) -> isize {
        let (rows, cols) = PointofIncidence::parse_to_bits(map);
        let row_result = PointofIncidence::match_lines(&rows, None);
        let col_result = PointofIncidence::match_lines(&cols, None);

        match row_result {
            r if r > -1 => (r + OFFSET) * 100,
            _ => col_result + OFFSET,
        }
    }

    fn process_gold(map: &[String]) -> isize {
        let (rows, cols) = PointofIncidence::parse_to_bits(map);
        let vars =
            PointofIncidence::create_variations(&rows, &cols, &map.len(), &map[0].len());
        let mut min_row = isize::MAX;
        let mut min_col = isize::MAX;

        let og_row_result = PointofIncidence::match_lines(&rows, None);
        let og_col_result = PointofIncidence::match_lines(&cols, None);

        for var in vars {
            let row_result = PointofIncidence::match_lines(&var.0, Some(og_row_result));
            let col_result = PointofIncidence::match_lines(&var.1, Some(og_col_result));

            if row_result >= 0 {
                min_row = std::cmp::min(min_row, row_result);
            }
            if col_result >= 0 {
                min_col = std::cmp::min(min_col, col_result);
            }
        }

        if (min_row >= 0) && min_row != isize::MAX {
            return (min_row + OFFSET) * 100;
        } else if (min_col >= 0) && min_col != isize::MAX {
            return min_col + OFFSET;
        }
        0
    }

    // new variation for each one cell bit toggle
    fn create_variations(
        rows: &[i32],
        cols: &[i32],
        height: &usize,
        width: &usize,
    ) -> Vec<(Vec<i32>, Vec<i32>)> {
        let mut vars = Vec::new();
        for y in 0..*height {
            for x in 0..*width {
                let mut new_rows = rows.to_vec();
                let mut new_cols = cols.to_vec();

                new_rows[y] ^= 1 << x;
                new_cols[x] ^= 1 << y;

                vars.push((new_rows, new_cols));
            }
        }
        vars
    }

    // return -1 if no match, otherwise the row number
    fn match_lines(lines: &[i32], ignore_this: Option<isize>) -> isize {
        let mut mirror: isize = -1;
        let length = lines.len() - OFFSET as usize;

        // find location of row pair
        for i in 0..length {
            if (lines[i] == lines[i + 1]) && Some(i as isize) != ignore_this {
                mirror = i as isize;
                let dist = min(i, length - i - 1);

                // if found, check if the pattern mirrors
                for cmp in 0..dist {
                    if lines[i - (cmp + 1)] != lines[i + 1 + (cmp + 1)] {
                        // not a perfect mirror
                        mirror = -1;
                        break;
                    }
                }
                if mirror > -1 {
                    return mirror;
                }
            }
        }
        mirror
    }

    fn parse_to_bits(map: &[String]) -> (Vec<i32>, Vec<i32>) {
        let mut rows: Vec<i32> = vec![0; map.len()];
        let mut cols: Vec<i32> = vec![0; map[0].len()];

        for (y, row) in map.iter().enumerate() {
            for (x, col) in row.chars().enumerate() {
                if col == '#' {
                    rows[y] |= 1 << x;
                    cols[x] |= 1 << y;
                }
            }
        }
        (rows, cols)
    }

    fn process_data<F>(data: &[String], processor: F) -> isize
    where
        F: Fn(&[String]) -> isize,
    {
        data.split(|row| row.is_empty()).map(processor).sum()
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2023/test/13.txt");
        let queue = PointofIncidence::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2023/real/13.txt");
        let queue = PointofIncidence::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(27502));
        assert_eq!(queue.gold(), TaskResult::Usize(31947));
    }
}
