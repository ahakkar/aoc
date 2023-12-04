#![allow(unused_variables)]
#![allow(unused_mut)]

use regex::Regex;
use std::collections::HashMap;
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
    let mut wins: HashMap<usize, usize> = HashMap::new();

    let start = Instant::now();
    process_wins(data, &mut wins);
    let duration = start.elapsed();
    println!("Time elapsed in process_wins() is: {:?}", duration);

    let start2 = Instant::now();
    // populate q initially with all rows
/*     for n in 0..=data.len()-1 {
        if wins[&n] > 0 {
            for _ in &n+1..= n + wins[&n] {
                sum += 1;
            }    
        }
    } */
    (0..=data.len()-1).for_each(|index| queue.push_back(index));

    while let Some(row_index) = queue.pop_front() {
        // if game has wins, add them to queue
        if wins[&row_index] > 0 {
            (&row_index+1..= row_index + wins[&row_index]).for_each(|index| queue.push_back(index));
        }
        sum += 1;
    }

    let duration2 = start2.elapsed();
    println!("Time elapsed in process() is: {:?}", duration2);
    println!("{}", sum); // 5422730, 194.71541ms
}

// count how many wins each game has
fn process_wins(data: &[&str], game_wins: &mut HashMap<usize, usize>) {
    for (row_idx, &row_str) in data.iter().enumerate() {
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
        
        game_wins.insert(row_idx, count_matching_numbers(&winning_nums, &lottery_nums));
    }
}

fn count_matching_numbers(vec1: &[i8], vec2: &[i8]) -> usize {
    let set1: HashSet<_> = vec1.iter().collect();
    vec2.iter().filter(|&n| set1.contains(n)).count()
}