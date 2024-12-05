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

use crate::{Fro, Solution};
use super::utils::*;

// Can add more shared vars here
pub struct Template {
    data: Vec<String>
}

// Can be used to implement fancier task-specific parsing
impl Fro for Template {
    fn fro(data: &str) -> Self{
        Self { data: data.split('\n').map(|line| line.to_string()).collect() }
    }
}

// Main solvers
impl Solution for Template {
    fn silver(&self) -> usize {
        0
    }
    

    fn gold(&self) -> usize {    
        0
    }

}

// For assisting functions
impl Template {
    
}


// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;   
    use super::*;   

    #[test]
    fn test() {  
        let test_data = read_data_from_file("input/test/06.txt"); 
        let queue = Template::fro(&test_data);        
  
        assert_eq!(queue.silver(), 0);
        assert_eq!(queue.gold(), 0);
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/real/06.txt");
        let queue = Template::fro(&real_data);        
  
        assert_eq!(queue.silver(), 0);
        assert_eq!(queue.gold(), 0);
    }
}
