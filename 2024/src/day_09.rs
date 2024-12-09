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
#![allow(unused_must_use)]

use std::fmt::{self, Display};

use crate::{Fro, Solution, TaskResult};
use super::utils::*;

#[derive(Debug, Clone)]
struct Block {
    id: usize,
    data: char,
}

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.data == '.' {
            writeln!(f, "Empty data block")
        } else {
            writeln!(f, "{:?}", self)
        }        
    }
}

// Can add more shared vars here
pub struct DiskFragmenter {
    data: Vec<Block>
}

// Can be used to implement fancier task-specific parsing
impl Fro for DiskFragmenter  {
    fn fro(input: &str) -> Self{
        let mut data: Vec<Block> = Vec::new();
        let mut i = 0;
        let mut data_idx = 0;
        input
            .chars()
            .for_each(|c| {
                let len = c as u8 -48;
                let mut data_added = false;
                for _ in (0..len) {
                    if i % 2 == 0 {
                        data.push(Block {id: data_idx, data: c});   
                        data_added = true;                     
                    } else {
                        data.push(Block {id: i, data: '.'});
                    }                    
                }   
                if data_added { data_idx += 1; }                    
                i += 1;
            });
        Self { data }
    }
}

// Main solvers
impl Solution for DiskFragmenter {
    fn silver(&self) -> TaskResult {
        let mut defrag = self.data.clone();
        // defrag.iter().for_each(|d| print!("{}", d));

        // first find the next free space, and hold it in index
        let mut next_free = 0;
        let mut next_end = defrag.len()-1;

        while let Some(idx) = Self::next_free_idx(&defrag, next_free) {
            // Find next data block from end 
            if let Some(end) = Self::next_end_idx(&defrag, next_end) {
                // Finish condition
                if idx >= end { break }
                
                // Swap the free space with data from the end
                defrag.swap(idx, end);
                
                    next_free = idx + 1;
                next_end = end - 1;
            } else {
                break; // No more data blocks to process
            }
        }         

        // defrag.iter().for_each(|d| print!("{}", d));

        TaskResult::Usize(
            defrag
                .iter()
                .enumerate()
                .map(|(i, val)| if val.data != '.' { val.id * i } else { 0 })
                .sum()
        )
    }
    

    fn gold(&self) -> TaskResult {    
        TaskResult::String("plaa".to_string())
    }

}

// For assisting functions
impl DiskFragmenter {
    fn next_free_idx(defrag: &[Block], start: usize) -> Option<usize> {
        (start..defrag.len()).find(|&i| defrag[i].data == '.')
    }

    fn next_end_idx(defrag: &[Block], end: usize) -> Option<usize> {
        (0..=end).rev().find(|&i| defrag[i].data != '.')
    }
}


// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;   
    use super::*;   

    #[test]
    fn test() {  
        let test_data = read_data_from_file("input/test/09.txt"); 
        let queue = DiskFragmenter::fro(&test_data);        
  
        assert_eq!(queue.silver(), TaskResult::Usize(1928));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/real/09.txt");
        let queue = DiskFragmenter::fro(&real_data);        
  
        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }
}
