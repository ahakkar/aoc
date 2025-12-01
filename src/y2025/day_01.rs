/*
 * 2025 Advent of Code with Rust
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
#![allow(clippy::never_loop)]
#![allow(clippy::useless_vec)]

use crate::{Fro, Solution, TaskResult};

// Can add more shared vars here
pub struct SecretEntrance {
    data: Vec<String>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for SecretEntrance {
    fn fro(input: &str) -> Self {
        Self {
            data: input.split('\n').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for SecretEntrance {
    fn silver(&self) -> TaskResult {
        let mut result = 0;
        let mut dial: i32 = 50;
        let lo: i32 = 0;
        let hi: i32 = 99;

        for row in &self.data {
            //println!("dir: {}, amount: {}", &row[0..1], &row[1..]);
            let amt = &row[1..].parse::<i32>().unwrap();

            match &row[0..1] {
                "L" => SecretEntrance::wrapping_sub(&mut dial, amt, &lo, &hi),
                "R" => SecretEntrance::wrapping_add(&mut dial, amt, &lo, &hi),
                _ => panic!("not expected direction"),
            };
            if dial == 0 {
                result += 1;
            }
            //println!("{}", dial);
        }

        TaskResult::Usize(result)
    }

    fn gold(&self) -> TaskResult {
        let mut result = 0;
        let mut dial: i32 = 50;
        let lo: i32 = 0;
        let hi: i32 = 99;

        for row in &self.data {
            //println!("dir: {}, amount: {}", &row[0..1], &row[1..]);
            let amt = &row[1..].parse::<i32>().unwrap();

            match &row[0..1] {
                "L" => {
                    SecretEntrance::wrapping_sub2(&mut dial, amt, &mut result, &lo, &hi)
                }
                "R" => {
                    SecretEntrance::wrapping_add2(&mut dial, amt, &mut result, &lo, &hi)
                }
                _ => panic!("not expected direction"),
            };
            //println!("{}", dial);
        }

        //println!("{}", dial);
        TaskResult::Usize(result)
    }
}

// For assisting functions
impl SecretEntrance {
    fn wrapping_add(a: &mut i32, b: &i32, lo: &i32, hi: &i32) {
        //println!("adding: {} {}", a, b);
        let width = hi - lo + 1;
        *a = ((*a + b) % width + width) % width;
    }

    fn wrapping_sub(a: &mut i32, b: &i32, lo: &i32, hi: &i32) {
        //println!("subbing: {} {}", a, b);
        let width = hi - lo + 1;
        *a = ((*a - b) % width + width) % width;
    }

    fn wrapping_add2(a: &mut i32, b: &i32, result: &mut usize, lo: &i32, hi: &i32) {
        //println!("adding: {} {}", a, b);
        for _ in 0..*b {
            *a += 1;
            if *a == 100 {
                *a = 0;
                *result += 1;
            }
        }
    }

    fn wrapping_sub2(a: &mut i32, b: &i32, result: &mut usize, lo: &i32, hi: &i32) {
        //println!("subbing: {} {}", a, b);
        for _ in 0..*b {
            *a -= 1;
            if *a == 0 {
                *result += 1;
            }
            if *a == -1 {
                *a = 99;
            }
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
        let mut test_data = read_data_from_file("input/2025/test/01.txt");
        let mut queue = SecretEntrance::fro(&test_data);
        assert_eq!(queue.silver(), TaskResult::Usize(3));

        test_data = read_data_from_file("input/2025/test/01b.txt");
        queue = SecretEntrance::fro(&test_data);
        assert_eq!(queue.gold(), TaskResult::Usize(10));

        test_data = read_data_from_file("input/2025/test/01.txt");
        queue = SecretEntrance::fro(&test_data);
        assert_eq!(queue.gold(), TaskResult::Usize(6));
    }

    /*     #[test]
    fn real() {
        let real_data = read_data_from_file("input/2025/real/01.txt");
        let queue = SecretEntrance::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    } */
}
