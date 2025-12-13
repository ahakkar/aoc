/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

use crate::{Fro, Solution, TaskResult};

// Can add more shared vars here
pub struct CosmicExpansion {
    data: Vec<String>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for CosmicExpansion {
    fn fro(input: &str) -> Self {
        let grid: Vec<Vec<char>> = data_as_chars(&data);
        let mut galaxies: Vec<Coord> = vec![];
        let mut empty_rows: Vec<isize> = vec![];
        let empty_cols: Vec<isize> = count_empty_cols(&grid);
        let mut is_row_empty;

        for (y, row) in grid.iter().enumerate() {
            is_row_empty = true;
            for (x, char) in row.iter().enumerate() {
                if char == &'#' {
                    is_row_empty = false;
                    galaxies.push(Coord::new(x as isize, y as isize));
                }
            }
            if is_row_empty {
                empty_rows.push(y.try_into().unwrap())
            }
        }
        Self {
            data: input.split('\n').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for CosmicExpansion {
    fn silver(&self) -> TaskResult {
        calc_pair_dist_sums(transform_coordinates(
            galaxies.clone(),
            &empty_rows,
            &empty_cols,
            2,
        ))
        .into()
    }

    fn gold(&self) -> TaskResult {
        calc_pair_dist_sums(transform_coordinates(
            galaxies.clone(),
            &empty_rows,
            &empty_cols,
            1_000_000,
        ))
        .into()
    }
}

// For assisting functions
impl CosmicExpansion {
    fn count_empty_cols(grid: &[Vec<char>]) -> Vec<isize> {
        let mut empty_cols: Vec<isize> = vec![];
        let num_cols = grid[0].len();

        for col_idx in 0..num_cols {
            if grid.iter().all(|row| row[col_idx] == '.') {
                empty_cols.push(col_idx.try_into().unwrap());
            }
        }
        empty_cols
    }

    fn transform_coordinates(
        mut galaxies: Vec<Coord>,
        empty_rows: &[isize],
        empty_cols: &[isize],
        step: isize,
    ) -> Vec<Coord> {
        for galaxy in galaxies.iter_mut() {
            galaxy.y += (empty_rows.iter().filter(|&&row| row < galaxy.y).count()
                as isize)
                * (step - 1);
            galaxy.x += (empty_cols.iter().filter(|&&col| col < galaxy.x).count()
                as isize)
                * (step - 1);
        }
        galaxies.clone()
    }

    fn calc_pair_dist_sums(galaxies: Vec<Coord>) -> i64 {
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

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2023/real/11.txt");
        let queue = CosmicExpansion::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(9623138));
        assert_eq!(queue.gold(), TaskResult::Usize(726820896326));
    }
}
