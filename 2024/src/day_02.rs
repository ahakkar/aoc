/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use std::cmp::Ordering;
use crate::utils::intvec_from_str;

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
            dir_cmp(&dir, &w[0], &w[1])
        })    
}

pub fn silver(data: &[String]) -> usize {
    data.iter()
        .filter(|s| is_safe(&intvec_from_str(s)))
        .count()   
}

// Check also if any permutation is safe by removing one element
pub fn gold(data: &[String]) -> usize {    
    data.iter()
        .filter(|s| {
            let row: Vec<isize> = intvec_from_str(s);
            is_safe(&row) || (0..row.len()).any(|i| {
                let mut copy = row.clone();
                copy.remove(i);
                is_safe(&copy)
            })
        })
        .count()
}

// run these with cargo test --bin main -- day_XX::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;
    use super::*;   
    

    #[test]
    fn test_test() {
        let test_data:Vec<String> = read_data_from_file("input/test/02.txt");
        assert_eq!(silver(&test_data), 2);
        assert_eq!(gold(&test_data), 4);
    }

    #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/02.txt");
        assert_eq!(silver(&test_data), 639);
    }

    #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/02.txt");
        assert_eq!(gold(&test_data), 674);
    }
}