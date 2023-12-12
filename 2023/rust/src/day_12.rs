/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_range_loop)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use std::collections::HashMap;

use super::utils::data_as_chars;

#[derive(Clone)]
struct Group {
    chars: Vec<char>,
    constraints: Vec<i8>,
    perms: i16,
    len: u8,
}

pub fn solve(data: Vec<String>) {
    let mut groups: Vec<Group> = vec![];
    let mut silver: i64 = 0;

    for row in data {
        groups.push(parse_to_group(row));
    }

    for g in groups {
        //println!("{:?} {:?} {}", g.chars, g.constraints, g.len);
        let mut current = String::new();
        silver += count_combinations(g, 0, current);
    }

    println!("Silver: {}", silver); // 7460
}

fn parse_to_group(row: String) -> Group {
    let (chars, constraints) = row
        .trim()
        .split_once(' ')
        .map(|(a, b)| (a.chars().collect::<Vec<char>>(), b.split(',')))
        .unwrap();

    Group{
        len: chars.len() as u8,
        chars,
        constraints: constraints.map(|s| s.parse::<i8>().unwrap()).collect::<Vec<i8>>(),
        perms: 0,        
    }
}

// forms patterns recursively and checks if they match constraints
fn count_combinations(group: Group, idx: u8, mut current: String) -> i64 {
    // base case
    if idx == group.len {
        match matches_constraints(current, group.constraints) {
            true => return 1,
            false => return 0,
        }
    }

    // replace ? with or . and continue recursion
    if group.chars.get(idx as usize).unwrap() == &'?' {
        return count_combinations(
            group.clone(),
            idx + 1,
            current.clone() + &'#'.to_string()
        ) + 
        count_combinations(
            group.clone(),
            idx + 1,
            current.clone() + &'.'.to_string()
        )
    }
    
    count_combinations(
        group.clone(),
        idx + 1,
        current + &group.chars.get(idx as usize).unwrap().to_string()
    )
}

fn matches_constraints(current: String, cst: Vec<i8>) -> bool {
    let mut cit = cst.iter().peekable();
    let mut cg_count = 0;

    for ch in current.chars() {
        match ch {
            '#' => cg_count += 1,
            '.' => { if cg_count > 0 {
                        if Some(&cg_count) != cit.next() { return false; }
                        cg_count = 0;
                    }
                }
             _  => return false,
        }
    }

    if cg_count > 0 && Some(&cg_count) != cit.next() { return false; }

    // All constraints were matched?
    cit.peek().is_none()
}

