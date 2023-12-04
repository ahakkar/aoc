use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("input/05simple.txt").unwrap();
    let data: Vec<&str> = input.lines().collect();

    process(&data);

    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}

fn process(data: &[&str]) {  
    let mut sum:i64 = 0;

    println!("{}", sum);
}

