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
    //println!("Silver: {}", silver(&data)); // 18724, 17517, 26168 too low, ok: 27502
    println!("Gold: {}", gold(&data)); // 22429 most likely too low // 38076 too high
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
fn match_gold_lines(lines: &[i32]) -> isize {
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

/* fn process_gold(map: &Vec<&String>) -> isize {
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


    // iterate through all variations with brute force
    for y in 0..rows.len() {
        for x in 0..cols.len() {
            print_bit_vec(&rows);
            //print_bit_vec(&cols);
            
            rows[y] ^= 1 << x;
            cols[x] ^= 1 << y;

            //println!("rows..");
            let row_result = match_gold_lines(&rows);
            //println!("cols..");
            let col_result = match_gold_lines(&cols);
            // println!("row sum: {} col sum: {}", row_result, col_result);   

            print_bit_vec(&rows);
            //print_bit_vec(&cols);            

            // Toggle the bit back
            rows[y] ^= 1 << x;
            cols[x] ^= 1 << y;

            if (row_result > -1 || col_result > -1) {
                if row_result > -1 {
                    return (row_result + OFFSET) * 100;
                } else if (col_result > -1) {
                     return col_result + OFFSET;
                };             
            }         
        }
    }

    panic!("Should find always a match..");
}
 */


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

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let mut new_rows = rows.clone();
            let mut new_cols = cols.clone();

            new_rows[y] ^= 1 << x;
            new_cols[x] ^= 1 << y;

            vars.push((new_rows, new_cols));
        }
    }
    // so slow..
    vars.retain(|(r, c)| *r != rows || *c != cols);

    let mut min_row = isize::MAX;
    let mut min_col = isize::MAX;
    let mut vars_len = vars.len();

    for var in vars {
        let row_result = match_lines(&var.0);
        let col_result = match_lines(&var.1);
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

    // perfect reflection across either a horizontal line between two rows
    // or across a vertical line between two columns.
    let row_result = match_lines(&rows);
    let col_result = match_lines(&cols);
    //println!("row sum: {} col sum: {}", row_result, col_result);   

/*     if row_result == col_result {
        println!("Error, {} == {}", row_result, col_result);
    } else  */
    
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
