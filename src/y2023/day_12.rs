/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

use crate::{Fro, Solution, TaskResult};
use std::collections::HashMap;

// Can add more shared vars here
pub struct HotSprings {
    data: Vec<String>,
}

#[derive(Clone)]
struct Group {
    chars: Vec<char>,
    constraints: Vec<i8>,
    len: usize,
}

// Can be used to implement fancier task-specific parsing
impl Fro for HotSprings {
    fn fro(input: &str) -> Self {
        Self {
            data: input.split('\n').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for HotSprings {
    fn silver(&self) -> TaskResult {
        let mut memo: HashMap<(Vec<i8>, String), usize> = HashMap::new();
        let mut groups: Vec<Group> = vec![];
        let mut silver: usize = 0;

        for row in &self.data {
            groups.push(HotSprings::parse_to_group(row.clone()));
        }

        for g in groups {
            //println!("{:?} {:?} {}", g.chars, g.constraints, g.len);
            let current = String::new();
            silver += HotSprings::count_combinations(&g, 0, current, &mut memo);
        }

        silver.into()
    }

    fn gold(&self) -> TaskResult {
        let mut memo: HashMap<(Vec<i8>, String), usize> = HashMap::new();
        let mut groups: Vec<Group> = vec![];
        let mut _gold: usize = 0;

        for row in &self.data {
            groups.push(HotSprings::unfold_to_group(row, 5));
        }

        for g in groups {
            println!("{:?} {:?} {}", g.chars, g.constraints, g.len);
            let current = String::new();
            _gold += HotSprings::count_combinations(&g, 0, current, &mut memo);
        }

        TaskResult::String("not done".to_string())
        //gold.into()
    }
}

// For assisting functions
impl HotSprings {
    fn parse_to_group(row: String) -> Group {
        let (chars, constraints) = row
            .trim()
            .split_once(' ')
            .map(|(a, b)| (a.chars().collect::<Vec<char>>(), b.split(',')))
            .unwrap();

        Group {
            len: chars.len(),
            chars,
            constraints: constraints
                .map(|s| s.parse::<i8>().unwrap())
                .collect::<Vec<i8>>(),
        }
    }

    fn unfold_to_group(row: &str, m: u8) -> Group {
        let (chars, cst) = row.trim().split_once(' ').unwrap();

        let chars_repeated: Vec<char> = std::iter::repeat_n(chars, m as usize)
            .collect::<Vec<&str>>()
            .join("?")
            .chars()
            .collect();

        let binding = std::iter::repeat_n(cst, m as usize)
            .collect::<Vec<&str>>()
            .join(",");
        let cst_repeated = binding.split(',');

        Group {
            len: chars_repeated.len(),
            chars: chars_repeated,
            constraints: cst_repeated
                .map(|s| s.parse::<i8>().unwrap())
                .collect::<Vec<i8>>(),
        }
    }

    // forms patterns recursively and checks if they match constraints
    fn count_combinations(
        group: &Group,
        idx: usize,
        current: String,
        memo: &mut HashMap<(Vec<i8>, String), usize>,
    ) -> usize {
        // base case
        if idx == group.len {
            let result =
                match HotSprings::matches_constraints(&current, &group.constraints) {
                    true => 1,
                    false => 0,
                };
            return result;
        }

        let key = (group.constraints.clone(), current.to_string()).clone();
        if let Some(&result) = memo.get(&key) {
            return result;
        }

        // replace ? with or . and continue recursion
        let result = if group.chars.get(idx).unwrap() == &'?' {
            HotSprings::count_combinations(
                group,
                idx + 1,
                current.clone() + &'#'.to_string(),
                memo,
            ) + HotSprings::count_combinations(
                group,
                idx + 1,
                current.clone() + &'.'.to_string(),
                memo,
            )
        } else {
            HotSprings::count_combinations(
                group,
                idx + 1,
                current.clone() + &group.chars.get(idx).unwrap().to_string(),
                memo,
            )
        };
        memo.insert(key.clone(), result);
        //key = &(group.constraints, current.to_string());
        //println!("idx: {} count: {} k: {:?} v:{}", idx, result, key, memo.get(&key).unwrap());
        result
    }

    fn matches_constraints(current: &str, cst: &[i8]) -> bool {
        let mut cit = cst.iter().peekable();
        let mut cg_count = 0;

        for ch in current.chars() {
            match ch {
                '#' => cg_count += 1,
                '.' => {
                    if cg_count > 0 {
                        if Some(&cg_count) != cit.next() {
                            return false;
                        }
                        cg_count = 0;
                    }
                }
                _ => return false,
            }
        }

        if cg_count > 0 && Some(&cg_count) != cit.next() {
            return false;
        }

        // All constraints were matched?
        cit.peek().is_none()
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2023/test/12.txt");
        let queue = HotSprings::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(21));
        assert_eq!(queue.gold(), TaskResult::Usize(525152));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2023/real/12.txt");
        let queue = HotSprings::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(7460));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }
}
