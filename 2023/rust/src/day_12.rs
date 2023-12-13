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
    len: usize,
}

pub fn solve(data: Vec<String>) {
    let mut memo: HashMap<(Vec<i8>, String), usize> = HashMap::new();
    let mut silver_groups: Vec<Group> = vec![];
    let mut gold_groups: Vec<Group> = vec![];
    let mut silver: usize = 0;
    let mut gold: usize = 0;

    for row in data {
        silver_groups.push(parse_to_group(row.clone()));
        gold_groups.push(unfold_to_group(row, 5));
    }

    for g in silver_groups {
        //println!("{:?} {:?} {}", g.chars, g.constraints, g.len);
        let mut current = String::new();
        silver += count_combinations(g, 0, current, &mut memo);
    }
    
    println!("Silver: {}", silver); // 7460
/*     memo.clear();

     for g in gold_groups {
        println!("{:?} {:?} {}", g.chars, g.constraints, g.len);
        let mut current = String::new();
        gold += count_combinations(g, 0, current, &mut memo);
    }

    
    println!("Gold: {}", gold); */
}

fn parse_to_group(row: String) -> Group {
    let (chars, constraints) = row
        .trim()
        .split_once(' ')
        .map(|(a, b)| (a.chars().collect::<Vec<char>>(), b.split(',')))
        .unwrap();

    Group{
        len: chars.len(),
        chars,
        constraints: constraints.map(|s| s.parse::<i8>().unwrap()).collect::<Vec<i8>>(),    
    }
}

fn unfold_to_group(row: String, m: u8) -> Group {
    let (chars, cst) = row
        .trim()
        .split_once(' ')
        .unwrap();

    let chars_repeated: Vec<char> = std::iter::repeat(chars)
        .take(m as usize)
        .collect::<Vec<&str>>()
        .join("?")
        .chars()
        .collect();

    let binding = std::iter::repeat(cst)
        .take(m as usize)
        .collect::<Vec<&str>>()
        .join(",");
    let cst_repeated = binding
        .split(',');

    Group{
        len: chars_repeated.len(),
        chars: chars_repeated,
        constraints: cst_repeated.map(|s| s.parse::<i8>().unwrap()).collect::<Vec<i8>>(),  
    }
}

// forms patterns recursively and checks if they match constraints
fn count_combinations(
    group: Group,
    idx: usize,
    mut current: String,
    memo: &mut HashMap<(Vec<i8>, String), usize>
) -> usize {
    // base case
    if idx == group.len {
        let result = match matches_constraints(&current, &group.constraints) {
            true => 1,
            false => 0,
        };
        return result;
    }

    let mut key = (group.constraints.clone(), current.to_string()).clone();
    if let Some(&result) = memo.get(&key) {
        return result;
    }

    // replace ? with or . and continue recursion
    let result = if group.chars.get(idx).unwrap() == &'?' {
        count_combinations(
            group.clone(),
            idx + 1,
            current.clone() + &'#'.to_string(),
            memo,
        ) + 
        count_combinations(
            group.clone(),
            idx + 1,
            current.clone() + &'.'.to_string(),
            memo,
        )
    }
    else {
        count_combinations(
            group.clone(),
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

