/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use std::collections::HashMap;

pub fn silver(data: &[String]) -> usize {
    let (mut a, mut b): (Vec<usize>, Vec<usize>) = data
        .iter()
        .map(|row| {
            let mut iter = row
                .split_ascii_whitespace()
                .map(|n| n.parse::<usize>().unwrap());

            (iter.next().unwrap(), iter.next().unwrap())
        })
        .unzip(); 

    a.sort();
    b.sort();

    a.iter()
        .zip(b.iter())
        .map(|(x, y)| x.abs_diff(*y))
        .sum()    
}

pub fn gold(data: &[String]) -> usize {
    let mut a: Vec<usize> = vec![];
    let mut b: HashMap<usize, usize> = HashMap::new();

    data
        .iter()
        .for_each(|row| {
            let mut iter = row
                .split_ascii_whitespace()
                .map(|n| n.parse::<usize>().unwrap());
            a.push(iter.next().unwrap());

            let num = iter.next().unwrap();
            *b.entry(num).or_insert(0) += 1;
        });

    a
        .iter()
        .map(|num| if let Some(val) = b.get(num) { num * val } else { 0 } )
        .sum()
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