/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

use crate::{
    Fro, Solution, TaskResult,
    util::{point2::Point2, utils::manhattan_distance},
};

// Can add more shared vars here
pub struct CosmicExpansion {
    galaxies: Vec<Point2>,
    empty_rows: Vec<i64>,
    empty_cols: Vec<i64>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for CosmicExpansion {
    fn fro(input: &str) -> Self {
        let grid: Vec<Vec<char>> =
            input.split('\n').map(|row| row.chars().collect()).collect();
        let mut galaxies: Vec<Point2> = vec![];
        let mut empty_rows: Vec<i64> = vec![];
        let empty_cols: Vec<i64> = CosmicExpansion::count_empty_cols(&grid);
        let mut is_row_empty;

        for (y, row) in grid.iter().enumerate() {
            is_row_empty = true;
            for (x, char) in row.iter().enumerate() {
                if char == &'#' {
                    is_row_empty = false;
                    galaxies.push(Point2::new(x as i64, y as i64));
                }
            }
            if is_row_empty {
                empty_rows.push(y.try_into().unwrap())
            }
        }
        Self {
            galaxies,
            empty_rows,
            empty_cols,
        }
    }
}

// Main solvers
impl Solution for CosmicExpansion {
    fn silver(&self) -> TaskResult {
        (CosmicExpansion::calc_pair_dist_sums(CosmicExpansion::transform_coordinates(
            self.galaxies.clone(),
            &self.empty_rows,
            &self.empty_cols,
            2,
        )) as usize)
            .into()
    }

    fn gold(&self) -> TaskResult {
        (CosmicExpansion::calc_pair_dist_sums(CosmicExpansion::transform_coordinates(
            self.galaxies.clone(),
            &self.empty_rows,
            &self.empty_cols,
            1_000_000,
        )) as usize)
            .into()
    }
}

// For assisting functions
impl CosmicExpansion {
    fn count_empty_cols(grid: &[Vec<char>]) -> Vec<i64> {
        let mut empty_cols: Vec<i64> = vec![];
        let num_cols = grid[0].len();

        for col_idx in 0..num_cols {
            if grid.iter().all(|row| row[col_idx] == '.') {
                empty_cols.push(col_idx.try_into().unwrap());
            }
        }
        empty_cols
    }

    fn transform_coordinates(
        mut galaxies: Vec<Point2>,
        empty_rows: &[i64],
        empty_cols: &[i64],
        step: i64,
    ) -> Vec<Point2> {
        for galaxy in galaxies.iter_mut() {
            galaxy.y += (empty_rows.iter().filter(|&&row| row < galaxy.y).count() as i64)
                * (step - 1);
            galaxy.x += (empty_cols.iter().filter(|&&col| col < galaxy.x).count() as i64)
                * (step - 1);
        }
        galaxies.clone()
    }

    fn calc_pair_dist_sums(galaxies: Vec<Point2>) -> i64 {
        let mut pairs = vec![];

        // Function to generate pairs
        for (i, coord1) in galaxies.iter().enumerate() {
            for coord2 in &galaxies[i + 1..] {
                pairs.push((coord1, coord2));
            }
        }

        pairs.iter().map(|(a, b)| manhattan_distance(a, b)).sum()
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2023/test/11.txt");
        let queue = CosmicExpansion::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(374));
        assert_eq!(queue.gold(), TaskResult::Usize(82000210));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2023/real/11.txt");
        let queue = CosmicExpansion::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(9623138));
        assert_eq!(queue.gold(), TaskResult::Usize(726820169514));
    }
}
