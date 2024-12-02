/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use std::collections::HashMap;

pub fn silver(data: &[String]) -> usize {
    let mut sum: usize = 0;  
    let mut a: Vec<usize> = vec![];
    let mut b: Vec<usize> = vec![];

    for row in data {
        let mut iter = row.split_ascii_whitespace();  
        a.push(
            iter.next().unwrap().parse::<usize>().unwrap()
        );
        b.push(
            iter.next().unwrap().parse::<usize>().unwrap()
        );
    }

    a.sort();
    b.sort();

    for i in 0..a.len() {
        if a[i] > b[i] {
            sum += a[i] - b[i];
        } else {
            sum += b[i] - a[i];
        }        
    }

    sum 
}

pub fn gold(data: &[String]) -> usize {
    let mut sum: usize = 0;    

    let mut a: Vec<usize> = vec![];
    let mut b: HashMap<usize, usize> = HashMap::new();

    for row in data {
        let mut iter = row.split_ascii_whitespace();  
        a.push(
            iter.next().unwrap().parse::<usize>().unwrap()
        );

        let num = iter.next().unwrap().parse::<usize>().unwrap();
        if let std::collections::hash_map::Entry::Vacant(e) = b.entry(num) {
            e.insert(1);
        } else  if let Some(x) = b.get_mut(&num) {
            *x += 1;           
        }        
    } 

    for num in a {
        if let Some(val) = b.get(&num) {
            sum += num * val;
        }
    }

    sum 
}

// run these with cargo test --bin main -- day_XX::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;
    use super::*;   

    #[test]
    fn test_test() {
        let test_data:Vec<String> = read_data_from_file("input/test/01.txt");
        assert_eq!(silver(&test_data), 11);
        assert_eq!(gold(&test_data), 31);
    }

    #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/01.txt");
        assert_eq!(silver(&test_data), 1879048);
    }

    #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/01.txt");
        assert_eq!(gold(&test_data), 21024792);
    }
}