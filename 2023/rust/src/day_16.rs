/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_range_loop)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use super::utils::{GridMap, Coord, data_as_chars};


pub fn solve(data: Vec<String>) {
    println!("Silver: {}", silver(&data));
    //println!("Gold: {}", gold(&data));
}

fn silver(data: &[String]) -> usize {
    let mut sum: usize = 0;    
    let map:GridMap = data_as_chars(data);


    sum 
}

/* fn gold(data: &[String]) -> usize {
    let mut sum: usize = 0;    

    for row in data {
           } 
    sum 
} */

// run these with cargo test --bin main -- day_13::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;
    use super::*;   

    #[test]
    fn test_test() {
        let test_data:Vec<String> = read_data_from_file("input/test/16.txt");
        assert_eq!(silver(&test_data), 46);
        //assert_eq!(gold(&test_data), 145);
    }

    #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/16.txt");
        assert_eq!(silver(&test_data), 510801);
    }

    #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/16.txt");
        //assert_eq!(gold(&test_data), 212763);
    }
}