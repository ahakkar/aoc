/*
 * 2025 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use std::collections::HashMap;

use crate::{Fro, Solution, TaskResult};
use ndarray::{Array2, ArrayView};

// Can add more shared vars here
pub struct TrashCompactor {
    data: Array2<usize>,
    gold_data: Vec<Vec<usize>>,
    ops: Vec<char>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for TrashCompactor {
    fn fro(input: &str) -> Self {
        let mut l_iter = input.lines();
        let height = l_iter.clone().count();
        // have to figure out the width first since it can not be dynamically sized..
        let width = l_iter
            .clone()
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .count();
        let ch_width = l_iter.clone().next().unwrap().chars().count();
        let mut data = Array2::zeros((0, width));

        // Collect data
        for _ in 0..height - 1 {
            let _ = data.push_row(ArrayView::from(
                &l_iter
                    .next()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<usize>>(),
            ));
        }
        // Collect ops
        let ops: Vec<char> = l_iter
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|c| c.parse().unwrap())
            .collect();

        // more spaghetti for gold
        // Rotate txt 90 degrees ccw and parse to matrix
        let mut grid: HashMap<(usize, usize), char> = HashMap::new();

        l_iter = input.lines();
        for col in 0..height - 1 {
            l_iter
                .next()
                .unwrap()
                .chars()
                .rev()
                .enumerate()
                .for_each(|(row, c)| {
                    grid.insert((row, col), c);
                });
        }

        let mut gold_data: Vec<Vec<usize>> = vec![];
        let mut temp_collection: Vec<usize> = vec![];

        for y in 0..ch_width {
            let mut num = 0;

            for x in 0..height - 1 {
                let ch = grid.get(&(y, x)).unwrap();
                if let Some(cur) = ch.to_digit(10) {
                    if num == 0 {
                        num = cur as usize;
                    } else {
                        num = num * 10 + cur as usize;
                    }
                }
            }
            if num > 0 {
                temp_collection.push(num);
            } else {
                gold_data.push(temp_collection.clone());
                temp_collection = vec![];
            }
        }
        gold_data.push(temp_collection);

        Self {
            data,
            gold_data,
            ops,
        }
    }
}

// Main solvers
impl Solution for TrashCompactor {
    fn silver(&self) -> TaskResult {
        self.ops
            .iter()
            .enumerate()
            .fold(0, |acc, (i, op)| match op {
                '+' => acc + self.data.column(i).sum(),
                '*' => acc + self.data.column(i).iter().product::<usize>(),
                _ => panic!("unsupported op"),
            })
            .into()
    }

    fn gold(&self) -> TaskResult {
        self.ops
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, op)| match op {
                '+' => acc + self.gold_data.get(i).unwrap().iter().sum::<usize>(),
                '*' => acc + self.gold_data.get(i).unwrap().iter().product::<usize>(),
                _ => panic!("unsupported op"),
            })
            .into()
    }
}

// For assisting functions
impl TrashCompactor {}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2025/test/06.txt");
        let queue = TrashCompactor::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(4277556));
        assert_eq!(queue.gold(), TaskResult::Usize(3263827));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2025/real/06.txt");
        let queue = TrashCompactor::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(5782351442566));
        assert_eq!(queue.gold(), TaskResult::Usize(10194584711842));
    }
}
