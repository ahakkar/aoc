#![allow(unused_variables)]
#![allow(unused_mut)]

use regex::Regex;
use std::collections::HashSet;
use std::collections::VecDeque;
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
    let mut sum:i64 = 0;
    let mut queue: VecDeque<usize> = VecDeque::new();

    // populate q initially with all rows
    (0..=data.len()-1).for_each(|index| queue.push_back(index));

    while let Some(row_index) = queue.pop_front() {
        process_row(data.get(row_index).unwrap(), &row_index, &mut queue);
        sum += 1;
        //println!("{}", sum);
    }

    println!("{}", sum);
}

fn process_row(row: &str, row_index: &usize, queue: &mut VecDeque<usize>) {
    // remove garbage from rows
    let start = row.find(':').map_or(0, |pos| pos + 1);
    let sliced_row = &row[start..];
    let mut parts = sliced_row.split('|');

    let winning: Vec<i8> = RE_ID.find_iter(parts.next().unwrap())
        .filter_map(|mat| mat.as_str().parse::<i8>().ok())
        .collect();
    let other: Vec<i8> = RE_ID.find_iter(parts.next().unwrap())
        .filter_map(|mat| mat.as_str().parse::<i8>().ok())
        .collect();     
    
    let more_wins = count_matching_numbers(&winning, &other);
    //print!("more wins: {}, ", more_wins);
    if more_wins > 0 {
        let range_start:usize = *row_index;
        //println!("start: {}, end: {} ", range_start+1, range_start + more_wins);
        (range_start+1..= range_start + more_wins).for_each(|index| queue.push_back(index));
    }
}

fn count_matching_numbers(vec1: &[i8], vec2: &[i8]) -> usize {
    let set1: HashSet<_> = vec1.iter().collect();
    vec2.iter().filter(|&n| set1.contains(n)).count()
}