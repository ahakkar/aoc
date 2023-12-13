/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

 #![allow(dead_code)]
 #![allow(unused_parens)]
 #![allow(unused_imports)]
 #![allow(unused_variables)]
 #![allow(unused_mut)]
 #![allow(unused_assignments)]
 #![allow(clippy::needless_return)]
 #![allow(clippy::needless_range_loop)]
 #![allow(clippy::explicit_counter_loop)]

use core::panic;
use std::cmp::min;
 
const OFFSET: isize = 1;
 
pub fn solve(data: Vec<String>) {
    println!("Silver: {}", silver(&data)); // 18724, 17517, 26168 too low, ok: 27502
    println!("Gold: {}", gold(&data)); // 22429 most likely too low // 38076 too high, 31947 ok
}

// return -1 if no match, otherwise the row number
fn match_lines(lines: &[i32]) -> isize {
    let mut mirror:isize = -1;
    let length = lines.len() - OFFSET as usize;
    let mut i = 0;
    //println!("{},  {:?}", length, lines);

    // find location of row pair
    for line in 0..length {        
        if lines[i] == lines[i+1] {                   
            mirror = i as isize; // save the row which was mirrored
            //println!("{} {}", i, length - i -1);
            let dist = min(i, length - i - 1);
            //println!("potential mirror on line {}, dist to border: {}", mirror, dist);     

            // if found, check if the pattern mirrors
            for cmp in 0..dist {
                //println!("cmp: {}, comparing lines: {:#018b} {:#018b}", cmp, lines[i-(cmp+1)], lines[i+1+(cmp+1)]);

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
        i += 1;
        //println!("{:#034b}", lines[i]);  
    }
    //println!("mirror on line {}, dist to closest border: {}", i+OFFSET, dist);  
    mirror    
}

// return -1 if no match, otherwise the row number
fn match_gold_lines(lines: &[i32], ignore_this: isize) -> isize {
    let mut mirror:isize = -1;
    let length = lines.len() - OFFSET as usize;
    let mut i = 0;
    //println!("{},  {:?}", length, lines);

    // find location of row pair
    for line in 0..length {        
        if (lines[i] == lines[i+1]) && (i != ignore_this as usize) {                   
            mirror = i as isize; // save the row which was mirrored
            //println!("{} {}", i, length - i -1);
            let dist = min(i, length - i - 1);
            //println!("potential mirror on line {}, dist to border: {}", mirror, dist);     

            // if found, check if the pattern mirrors
            for cmp in 0..dist {
                //println!("cmp: {}, comparing lines: {:#018b} {:#018b}", cmp, lines[i-(cmp+1)], lines[i+1+(cmp+1)]);

                if lines[i-(cmp+1)] != lines[i+1+(cmp+1)] {
                    //println!("not a perfect mirror");
                    // not a perfect mirror
                    mirror = -1;
                    //println!("value of mirror: {}", mirror);
                    break;
                }                
            }             
            if mirror > -1 {
                //println!("value of mirror: {}", mirror);
                return mirror;
            }       
        }
        i += 1;
        //println!("{:#034b}", lines[i]);  
    }
    //println!("value of mirror: {}", mirror);
    mirror    
}
fn print_bit_vec(vec: &[i32]) {
    for line in vec {
        println!("{:#034b}", line); 
    }
    println!();
}

fn process_gold(map: &Vec<&String>) -> isize {
    let mut rows: Vec<i32> = vec![0; map.len()];
    let mut cols: Vec<i32> = vec![0; map[0].len()];
    let mut earliest_result: isize = -1;
    let mut vars: Vec<(Vec<i32>, Vec<i32>)> = vec![];

    // parse map to bits
    for (y, row) in map.iter().enumerate() {
        for (x, col) in row.chars().enumerate() {
            if col == '#' {
                rows[y] |= 1 << x;
                cols[x] |= 1 << y;
            }
        }
    }

    let og_row_result = match_lines(&rows);
    let og_col_result = match_lines(&cols);

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let mut new_rows = rows.clone();
            let mut new_cols = cols.clone();

            new_rows[y] ^= 1 << x;
            new_cols[x] ^= 1 << y;

            vars.push((new_rows, new_cols));
        }
    }

    let mut min_row = isize::MAX;
    let mut min_col = isize::MAX;
    let mut vars_len = vars.len();

    for var in vars {
        let row_result = match_gold_lines(&var.0, og_row_result);
        let col_result = match_gold_lines(&var.1, og_col_result);
        //print_bit_vec(&var.0);
        //print_bit_vec(&var.1);

        //println!("row result: {} col result: {}", row_result, col_result);  
        if row_result >= 0 {
            min_row = std::cmp::min(min_row, row_result);
        }    
        if col_result >= 0 {
            min_col = std::cmp::min(min_col, col_result);
        }     
    }

    //print!("variations: {}, min row: {} min col: {}", vars_len, min_row, min_col);  

    if (min_row >= 0) && min_row != isize::MAX {
        //println!(", returning value: {}", (min_row + OFFSET) * 100);
        return (min_row + OFFSET) * 100;
    } else if (min_col >= 0) && min_col != isize::MAX {
        //println!(", returning value: {}", min_col + OFFSET);
        return min_col + OFFSET;
    }

    0     
}

fn gold(data: &[String]) -> isize {
    let mut sum: isize = 0; 
    let mut map: Vec<&String> = vec![];

    // parse maps from data
    for row in data {        
        if !row.is_empty() {
            map.push(row);
        } else {
            //println!("processing map..");
            //println!("{:?}", map);
            sum += process_gold(&map);
            map.clear();
        }        
    }

    if !map.is_empty() {
        //println!("Processing remaining map...");
        sum += process_gold(&map);
    }
    
    sum 
}

fn process_silver(map: &Vec<&String>) -> isize {
    let mut rows: Vec<i32> = vec![0; map.len()]; 
    let mut cols: Vec<i32> = vec![0; map[0].len()]; 

    // parse map to bits
    for (y, row) in map.iter().enumerate() {
        for (x, col) in row.chars().enumerate() {            
            if col == '#' {
                rows[y] |= 1 << x;
                cols[x] |= 1 << y;
            }
        }        
    }

    let row_result = match_lines(&rows);
    let col_result = match_lines(&cols);
    
    if (row_result > -1) {
        return (row_result + OFFSET) * 100;
    }
    col_result + OFFSET    
}

fn silver(data: &[String]) -> isize {
    let mut sum: isize = 0; 
    let mut map: Vec<&String> = vec![];

    // parse maps from data
    for row in data {        
        if !row.is_empty() {
            map.push(row);
        } else {
            //println!("processing map..");
            //println!("{:?}", map);
            sum += process_silver(&map);
            map.clear();
        }        
    }

    if !map.is_empty() {
        //println!("Processing remaining map...");
        sum += process_silver(&map);
    }
    
    sum 
}

// run these with cargo test --bin main -- day_13::tests
#[cfg(test)]
mod tests {
    use crate::utils::{self, read_data_from_file};
    use super::*;   

    #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/13.txt");
        assert_eq!(silver(&test_data), 27502);
    }

    #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/13.txt");
        assert_eq!(gold(&test_data), 31947);
    }
}