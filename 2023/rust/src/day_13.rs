/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */
use std::cmp::min;
 
const OFFSET: isize = 1;

// return -1 if no match, otherwise the row number
fn match_lines(lines: &[i32], ignore_this: Option<isize>) -> isize {
    let mut mirror:isize = -1;
    let length = lines.len() - OFFSET as usize;

    // find location of row pair
    for i in 0..length {        
        if (lines[i] == lines[i+1]) && Some(i as isize) != ignore_this {                   
            mirror = i as isize;
            let dist = min(i, length - i - 1);    

            // if found, check if the pattern mirrors
            for cmp in 0..dist {
                if lines[i-(cmp+1)] != lines[i+1+(cmp+1)] {
                    // not a perfect mirror
                    mirror = -1;
                        break;
                }                
            }             
            if mirror > -1 {
                  return mirror;
            }       
        }
    }
    mirror    
}

fn parse_to_bits(map: &[String]) -> (Vec<i32>, Vec<i32>) {
    let mut rows: Vec<i32> = vec![0; map.len()];
    let mut cols: Vec<i32> = vec![0; map[0].len()];
        
    for (y, row) in map.iter().enumerate() {
        for (x, col) in row.chars().enumerate() {
            if col == '#' {
                rows[y] |= 1 << x;
                cols[x] |= 1 << y;
            }
        }
    }
    (rows, cols)
}

fn process_data<F>(data: &[String], processor: F) -> isize
where
    F: Fn(&[String]) -> isize,
{
    data.split(|row| row.is_empty())
        .map(processor)
        .sum()
}

fn silver(map: &[String]) -> isize {
    let (rows, cols) = parse_to_bits(map);
    let row_result = match_lines(&rows, None);
    let col_result = match_lines(&cols, None);
    
    if row_result > -1 {
        return (row_result + OFFSET) * 100;
    }
    col_result + OFFSET    
}

fn gold(map: &[String]) -> isize {
    let (rows, cols) = parse_to_bits(map);
    let mut vars: Vec<(Vec<i32>, Vec<i32>)> = vec![];
    let mut min_row = isize::MAX;
    let mut min_col = isize::MAX;

    let og_row_result = match_lines(&rows, None);
    let og_col_result = match_lines(&cols, None);

    // new variation for each one cell bit toggle
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let mut new_rows = rows.clone();
            let mut new_cols = cols.clone();

            new_rows[y] ^= 1 << x;
            new_cols[x] ^= 1 << y;

            vars.push((new_rows, new_cols));
        }
    }

    for var in vars {
        let row_result = match_lines(&var.0, Some(og_row_result));
        let col_result = match_lines(&var.1, Some(og_col_result));

        if row_result >= 0 {
            min_row = std::cmp::min(min_row, row_result);
        }    
        if col_result >= 0 {
            min_col = std::cmp::min(min_col, col_result);
        }     
    }

    if (min_row >= 0) && min_row != isize::MAX {
        return (min_row + OFFSET) * 100;
    } else if (min_col >= 0) && min_col != isize::MAX {
        return min_col + OFFSET;
    }
    0     
}
 
pub fn solve(data: Vec<String>) {
    println!("Silver: {}", process_data(&data, silver)); // 27502
    println!("Gold: {}", process_data(&data, gold)); // 31947
}

// run these with cargo test --bin main -- day_13::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;
    use super::*;   

    #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/13.txt");
        assert_eq!(process_data(&test_data, silver), 27502);
    }

    #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/13.txt");
        assert_eq!(process_data(&test_data, gold), 31947);
    }
}