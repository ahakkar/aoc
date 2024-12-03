/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_range_loop)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use std::vec;

use super::utils::*;
use regex::Regex;

fn parse_tuple(s: &str) -> (usize, usize) {
    s
        .get(4..s.len() - 1)
        .unwrap()
        .split_once(',')
        .map(|(a, b)| (a.trim().parse().unwrap(), b.trim().parse().unwrap()))
        .unwrap()
}

fn parse_row(row: &str) -> Vec<(usize, usize)> {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    re
        .find_iter(row)
        .map(|m| {
            let s = m.as_str(); 
            parse_tuple(s)   
        })  
        .collect()
}

pub fn silver(data: &[String]) -> usize {
    
    let mut matches: Vec<(usize, usize)> = vec![];
    for row in data {
        matches.extend(parse_row(row));
    }
    
    matches
        .iter()
        .map(|t| t.0 * t.1)
        .sum()
    // 31818595 too low
}

fn parse_row2(row: &str) -> Vec<String> {
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don\'t\(\)").unwrap();
    re
        .find_iter(row)
        .map(|m| m.as_str().to_string())
        .collect()
}

pub fn gold(data: &[String]) -> usize {
    let mut sum: usize = 0;
    let mut matches: Vec<String> = vec![];
    for row in data {
        matches.extend(parse_row2(row));
    }

    let mut mode = true;
    for entry in &matches {
        match entry.as_str() {
            "don't()" => mode = false,
            "do()" => mode = true,
            _ => (),
        }

        if mode {
            match entry.as_str() {
                "don't()" => (),
                "do()" => (),
                _ => {
                    let t = parse_tuple(entry);
                    sum += t.0 * t.1;
                }
            }
        }
    }
    sum 
}

// run these with cargo test --bin main -- day_XX::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;
    use super::*;   

    #[test]
    fn test_test() {
        let silver_data:Vec<String> = read_data_from_file("input/test/03s.txt");
        let gold_data:Vec<String> = read_data_from_file("input/test/03g.txt");
        assert_eq!(silver(&silver_data), 161);
        assert_eq!(gold(&gold_data), 48);
    }

    #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/03.txt");
        assert_eq!(silver(&test_data), 175615763);
    }

    #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/03.txt");
        assert_eq!(gold(&test_data), 74361272);
    }
}