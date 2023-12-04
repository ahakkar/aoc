#![allow(unused_variables)]
#![allow(unused_mut)]

use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE_ID: regex::Regex = Regex::new(r"\d+").unwrap();
    static ref RE_SYMBOL: regex::Regex = Regex::new(r"[*]").unwrap();
}

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("input/04puzzle.txt").unwrap();
    let data: Vec<&str> = input.lines().collect();

    process(&data);

    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

fn process(data: &[&str]) {  
    // Store counts of each card
    let mut counts = vec![1; data.len()];
    data.iter().enumerate().for_each(|(i, row_str)| {
        (0..process_score(row_str)).for_each(|n| counts[i + n + 1] += counts[i]);
    });   

    // Sum the count of all cards
    println!("{}", counts.iter().sum::<usize>());
}

// count how many wins each game has
fn process_score(row_str: &str) -> usize {
    // remove garbage from rows
    let start = row_str.find(':').map_or(0, |pos| pos + 1);
    let sliced_row = &row_str[start..];
    let mut parts = sliced_row.split('|');

    let winning_nums: Vec<i8> = RE_ID.find_iter(parts.next().unwrap())
        .filter_map(|mat| mat.as_str().parse::<i8>().ok())
        .collect();
    let lottery_nums: Vec<i8> = RE_ID.find_iter(parts.next().unwrap())
        .filter_map(|mat| mat.as_str().parse::<i8>().ok())
        .collect();     
    
    count_matching_numbers(&winning_nums, &lottery_nums)    
}

fn count_matching_numbers(vec1: &[i8], vec2: &[i8]) -> usize {
    let set1: HashSet<_> = vec1.iter().collect();
    vec2.iter().filter(|&n| set1.contains(n)).count()
}