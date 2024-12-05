/*
 * 2024 Advent of Code with Rust
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

use super::utils::*;

pub fn solve(data: Vec<String>) {
    let parsed_data = parse_data(&data);       

    println!("Silver: {}", silver(&parsed_data));
    println!("Gold: {}", gold(&parsed_data));
}

fn silver(data: &[String]) -> usize {
    let mut sum: usize = 0;    

    sum 
}

fn gold(data: &[String]) -> usize {
    let mut sum: usize = 0;    

    sum 
}

fn parse_data(data: &[String]) -> Vec<String> {
    let data: Vec<String> = Vec::new();
    data

}

// cargo test --bin main -- day_XX::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;   
    use super::*;   

    #[test]
    fn test_test() {  
        let test_data = read_data_from_file("input/test/06.txt");
        let data = parse_data(&test_data);       
        assert_eq!(silver(&data), 143);
        //assert_eq!(gold(&data), 123);
    }

    #[test]
    fn test_real() {
        let real_data = read_data_from_file("input/real/06.txt");
        let data = parse_data(&real_data);  
        assert_eq!(silver(&data), 5713);
        //assert_eq!(gold(&data), 5180);
    }
}