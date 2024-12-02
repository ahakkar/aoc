/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

pub fn solve(data: Vec<String>) {    
    println!("Silver: {}", silver(&data));
    println!("Gold: {}", gold(&data));
}

fn is_desc_window(w: & [isize]) -> bool {
    (w[0] > w[1]) && 
    ((w[0] - w[1]).abs() > 0) &&
    ((w[0] - w[1]).abs() < 4) 
}

fn is_asc_window(w: & [isize]) -> bool {
    (w[0] < w[1]) && 
    ((w[0] - w[1]).abs() > 0) &&
    ((w[0] - w[1]).abs() < 4) 
}


fn silver(data: &[String]) -> usize {
    data
        .iter()
        .filter(|row| {
            let numbers: Vec<isize> = row
                .split_ascii_whitespace()
                .map(|s| s.parse::<isize>().unwrap())
                .collect();
 
            numbers
                .windows(2)
                .all(is_desc_window) || 
            numbers
                .windows(2)
                .all(is_asc_window) 
        })
        .count()   
}

#[derive(Debug)]
enum ListOrder {
    Ascending,
    Descending,
    Unknown,
}

fn gold(data: &[String]) -> usize {
    let mut sum = 0;
    let reports: Vec<Vec<isize>> = data
        .iter()
        .map(|row| {
            row.split_ascii_whitespace()
                .map(|s| s.parse::<isize>().unwrap())
                .collect()
        })
        .collect();

    for row in reports {
        let dir = match row.first().cmp(&row.last()) {
            std::cmp::Ordering::Greater => ListOrder::Descending,
            std::cmp::Ordering::Less => ListOrder::Ascending,
            _ => ListOrder::Unknown,
        };

        let is_valid_two = match dir {
            ListOrder::Ascending => row.windows(2).all(is_asc_window),
            ListOrder::Descending => row.windows(2).all(is_desc_window),
            _ => false,
        };

        if is_valid_two {
            sum += 1;
            continue;
        }

        // Generate permutations by removing one element at a time
        let mut found_valid_permutation = false;
        for i in 0..row.len() {
            let mut copy = row.clone();
            copy.remove(i);

            match dir {
                ListOrder::Ascending => {
                    if copy.windows(2).all(is_asc_window) {
                        found_valid_permutation = true;
                        break;
                    }
                }
                ListOrder::Descending => {
                    if copy.windows(2).all(is_desc_window) {
                        found_valid_permutation = true;
                        break;
                    }
                }
                _ => {}
            }
        }

        if found_valid_permutation {
            sum += 1;
        }
    }

    // 685 too high, 665 too low, 674 ok
    sum
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