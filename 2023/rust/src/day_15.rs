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
 
 
pub fn solve(data: Vec<String>) {
    //println!("Silver: {}", silver(&vec![String::from("HASH")]));
    println!("Silver: {}", silver(&data)); // 510801
    //println!("Gold: {}", gold(&data));
 }
 
fn silver(data: &[String]) -> usize {
    let mut sum: usize = 0;
    let mut cv: usize = 0;

    let hashes = data[0]
        .split(',');

    for hash in hashes {
        cv = 0;
        for c in hash.chars() {            
            cv += c as usize;
            cv *= 17;
            cv %= 256;
        }
        sum += cv;        
    }
    sum 
}

/* fn gold(data: &Vec<String>) -> i64 {
    let mut sum: i64 = 0;    

    for row in data {
        } 
    sum 
} */