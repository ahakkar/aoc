/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use crate::{Fro, Solution, TaskResult};

type Rules = HashMap<usize, HashSet<usize>>;
type Print = Vec<Vec<usize>>;

pub struct PrintQueue {
    rules: Rules,
    print: Print,
}

impl Fro for PrintQueue {
    fn fro(data: &str) -> Self {
        let mut toggle = false;
        let mut rules: Rules = HashMap::new();
        let mut print: Print = vec![];
        let data: Vec<String> = data.split('\n').map(|line| line.to_string()).collect();

        for row in data {
            if row.is_empty() {
                toggle = true;
                continue;
            }
            if toggle {
                print.push(
                    row.split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect(),
                );
            } else if let Some((a, b)) = row.split_once('|') {
                let a = a.trim().parse::<usize>().unwrap();
                let b = b.trim().parse::<usize>().unwrap();
                rules.entry(a).or_default().insert(b);
            }
        }

        Self { rules, print }
    }
}

impl Solution for PrintQueue {
    // For each page in pagelist, check the rules for page.
    // If any of the pages in rules[page] are already in printed,
    // the order is invalid. Otherwise, add page to printed.
    fn silver(&self) -> TaskResult {
        TaskResult::Usize(
            self.print
                .iter()
                .filter(|pages| {
                    pages.is_sorted_by(|a, b| {
                        Self::custom_sort(a, b, &self.rules) != Ordering::Greater
                    })
                })
                .map(|pages| pages.get(pages.len() / 2).unwrap())
                .sum(),
        )
    }

    fn gold(&self) -> TaskResult {
        TaskResult::Usize(
            self.print
                .clone()
                .iter_mut()
                .filter(|pages| {
                    !pages.is_sorted_by(|a, b| {
                        Self::custom_sort(a, b, &self.rules) != Ordering::Greater
                    })
                })
                .map(|pages| {
                    pages.sort_by(|a, b| Self::custom_sort(a, b, &self.rules));
                    pages.get(pages.len() / 2).unwrap()
                })
                .sum(),
        )
    }
}

impl PrintQueue {
    fn custom_sort(a: &usize, b: &usize, rules: &Rules) -> Ordering {
        if a == b {
            return Ordering::Equal;
        }
        if let Some(rules) = rules.get(a)
            && rules.contains(b)
        {
            return Ordering::Less;
        }
        Ordering::Greater
    }
}

// cargo test --bin main -- day_XX::tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TaskResult, util::utils::read_data_from_file};

    #[test]
    fn test_test() {
        let test_data = read_data_from_file("input/2024/test/05.txt");
        let queue = PrintQueue::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(143));
        assert_eq!(queue.gold(), TaskResult::Usize(123));
    }

    #[test]
    fn test_real() {
        let real_data = read_data_from_file("input/2024/real/05.txt");
        let queue = PrintQueue::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(5713));
        assert_eq!(queue.gold(), TaskResult::Usize(5180));
    }
}
