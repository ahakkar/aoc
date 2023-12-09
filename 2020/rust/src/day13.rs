/*
 * 2020 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

pub fn solve(data: Vec<String>) {
    println!("Silver: {}", silver(&data));
    //println!("Gold: {}", gold(&data));
}

fn silver(data: &Vec<String>) -> i64 {
    for row in data {
        println!("{}", row);
    }
    0
}

/* fn gold(data: &Vec<String>) -> i64 {
    0
}
 */