/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/
use crate::{utils::intvec_from_str, Fro, Solution, TaskResult};
use std::cmp::Ordering;

pub struct RedNosedReports {
    data: Vec<String>
}

impl Fro for RedNosedReports {
    fn fro(data: &str) -> Self{
        Self { data: data.split('\n').map(|line| line.to_string()).collect() }
    }
}

impl Solution for RedNosedReports {
    fn silver(&self) -> TaskResult {
        TaskResult::Usize(self.data.iter()
            .filter(|s| Self::is_safe(&intvec_from_str(s)))
            .count())
    }
    
    // Check also if any permutation is safe by removing one element
    fn gold(&self) -> TaskResult {    
        TaskResult::Usize(self.data.iter()
            .filter(|s| {
                let row: Vec<isize> = intvec_from_str(s);
                Self::is_safe(&row) || (0..row.len()).any(|i| -> bool {
                    let mut copy = row.clone();
                    copy.remove(i);
                    Self::is_safe(&copy)
                })
            })
            .count())
    }
}

impl RedNosedReports {
    fn dir_cmp(dir: &Ordering, a: &isize, b: &isize) -> bool {
        match dir {
            Ordering::Greater => a > b,
            Ordering::Less => a < b,
            _ => false,
        }
    }

    fn is_safe(row: &[isize]) -> bool {
        let dir = row.first().unwrap().cmp(row.last().unwrap());
        row.windows(2)
            .all(|w| {
                (1..=3).contains(&(w[0] - w[1]).abs()) && 
                Self::dir_cmp(&dir, &w[0], &w[1])
            })    
    }
}



// run these with cargo test --bin main -- day_XX::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;   
    use super::*;   

    #[test]
    fn test_test() {  
        let test_data = read_data_from_file("input/test/02.txt"); 
        let queue = RedNosedReports::fro(&test_data);        
  
        assert_eq!(queue.silver(), TaskResult::Usize(2));
        assert_eq!(queue.gold(), TaskResult::Usize(4));
    }

    #[test]
    fn test_real() {
        let real_data = read_data_from_file("input/real/02.txt");
        let queue = RedNosedReports::fro(&real_data);        
  
        assert_eq!(queue.silver(), TaskResult::Usize(639));
        assert_eq!(queue.gold(), TaskResult::Usize(674));
    }
}
