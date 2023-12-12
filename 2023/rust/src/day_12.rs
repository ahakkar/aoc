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

use super::utils::data_as_chars;

struct Group {
    chars: Vec<char>,
    grouping: Vec<i8>,
    perms: i16,
}

pub fn solve(data: Vec<String>) {
    let mut groups: Vec<Group> = vec![];
    let mut silver: i64 = 0;

    for row in data {
        groups.push(parse_to_group(row));
    }

    for g in groups {
        silver += permute(g);
    }

    println!("Silver: {}", silver);
}

fn parse_to_group(row: String) -> Group {
    let (chars, grouping) = row
        .trim()
        .split_once(' ')
        .map(|(a, b)| (a.chars().collect(), b.split(',')))
        .unwrap();

    Group{
        chars,
        grouping: grouping.map(|s| s.parse::<i8>().unwrap()).collect(),
        perms: 0
    }
}

fn permute(group: Group) -> i64 {
    println!("{:?}, {:?}", group.chars, group.grouping);
    0
}