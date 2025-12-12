use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_ID: regex::Regex = Regex::new(r"\d{1,3}").unwrap();
    static ref RE_SYMBOL: regex::Regex = Regex::new(r"[*]").unwrap();
}

use crate::{Fro, Solution, TaskResult};

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
        let mut sum = 0;

        for (y, row) in data.iter().enumerate() {
            let ids_iter = RE_ID.find_iter(row);

            // Add the id to sum if it has a symbol as neighbour
            for iter in ids_iter {
                if check_neighbours(twod_grid, &y, &iter) {
                    sum += iter.as_str().parse::<usize>().unwrap();
                }
            }
        }
        sum.into()
    }

    fn gold(&self) -> TaskResult {
        let mut sum = 0;
        for (row, row_str) in data.iter().enumerate() {
            for gear in RE_SYMBOL.find_iter(row_str) {
                sum += check_neighbours(data, &row, &gear.start());
            }
        }
        sum.into()
    }
}

// For assisting functions
impl GearRatios {
    /**
     *   0  1  2  3  4  5  6  7
     *   8  9 10  * 12 13 14 15
     *  16 17 18 19 20 21 22 23
     */
    fn check_neighbours(data: &[&str], row: &usize, col: &usize) -> i64 {
        let start = col.saturating_sub(3);
        let end = start + 7;

        let mut neighbours: String = String::new();
        let mut n1: i64 = 0;
        let mut n2: i64 = 0;

        // iterate through rows -1, 0, 1
        for i in 0..3 {
            neighbours += &data[row + i - 1][start..end];
            neighbours.push('|');
        }

        // check if any numbers are next to the gear in the flattened view
        for num in RE_ID.find_iter(&neighbours) {
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
