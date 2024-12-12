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

fn process(data: &Vec<&str>) {    
    let mut sum:i64 = 0;

    for row in data {
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
        
        let result: u32 = u32::try_from(count_matching_numbers(&winning, &other)).unwrap();
        match result {
            0 => (),
            _ => sum += 2_i64.pow(result - 1),
        }   
    }

    println!("{}", sum); // 84900879
}

fn count_matching_numbers(vec1: &[i8], vec2: &[i8]) -> usize {
    let set1: HashSet<_> = vec1.iter().collect();
    vec2.iter().filter(|&n| set1.contains(n)).count()
}