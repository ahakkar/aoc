/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use crate::utils::{intvec_from_str, ListOrder};

pub fn solve(data: Vec<String>) {    
    println!("Silver: {}", silver(&data));
    println!("Gold: {}", gold(&data));
}

fn list_tentative_order(first: &isize, last: &isize) -> ListOrder {
    match first.cmp(last) {
        std::cmp::Ordering::Greater => ListOrder::Descending,
        std::cmp::Ordering::Less => ListOrder::Ascending,
        _ => ListOrder::Unknown,
    }
}

fn dir_cmp(dir: &ListOrder, a: &isize, b: &isize) -> bool {
    match dir {
        ListOrder::Ascending => a < b,
        ListOrder::Descending => a > b,
        _ => false,
    }
}

fn is_safe(row: &[isize]) -> bool {
    let dir = list_tentative_order(row.first().unwrap(), row.last().unwrap());
    row
        .windows(2)
        .all(|w| {
            (1..=3).contains(&(w[0] - w[1]).abs()) && 
            dir_cmp(&dir, &w[0], &w[1])
        })    
}

fn silver(data: &[String]) -> usize {
    data
        .iter()
        .filter(|row| is_safe(&intvec_from_str(row)))
        .count()   
}

// Check also if any permutation is safe by removing one element
fn gold(data: &[String]) -> usize {    
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