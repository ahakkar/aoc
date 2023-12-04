use std::fs;
use std::time::Instant;

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

    for (i, row_str) in data.iter().enumerate() {
        for n in 0..process_score(row_str) {
            counts[i + n + 1] += counts[i];
        }
    }   

    // Sum the count of all cards
    println!("{}", counts.iter().sum::<usize>()); // 5422730
}

// count how many wins each game has
fn process_score(row_str: &str) -> usize {
    // discard all before ': ' in rows
    let (winning_nums, lottery_nums) = 
        row_str.split_once(": ").unwrap().1.split_once(" | ").unwrap();

    let winning_nums = winning_nums
        .split(' ')
        .filter_map(|n| n.parse::<usize>().ok())
        .collect::<std::collections::HashSet<_>>();

    // Count and return the matching numbers between winning & lottery
    lottery_nums.split(' ')
        .filter_map(|n| n.parse::<usize>().ok())
        .filter_map(|n| winning_nums.contains(&n).then_some(1))
        .sum()
}
