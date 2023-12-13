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

use std::cmp::min;
 
const OFFSET: isize = 1;
 
pub fn solve(data: Vec<String>) {
    println!("Silver: {}", silver(&data)); // 18724 too low // 17517
    //println!("Gold: {}", gold(&data));
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
            println!("potential mirror on line {}, dist to border: {}", mirror, dist);     

            // if found, check if the pattern mirrors
            for cmp in 0..dist {
                //println!("cmp: {}, comparing lines: {:#018b} {:#018b}", cmp, lines[i-(cmp+1)], lines[i+1+(cmp+1)]);

                if lines[i-(cmp+1)] != lines[i+1+(cmp+1)] {
                    // not a perfect mirror
                    break;
                }
            }

            // perfect mirror found
            //println!("mirror on line {}, dist to closest border: {}", i+OFFSET, dist);   
            return mirror + OFFSET;            
        }
        i += 1;
        //println!("{:#034b}", lines[i]);  
    }
    return mirror + OFFSET;  
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
    let row_sum = match_lines(&rows) * 100;
    //println!();
    let col_sum = match_lines(&cols);

    if row_sum == col_sum {
        println!("Error, {} == {}", row_sum, col_sum);
    }

    //    println!("row sum: {} col sum: {}", row_sum, col_sum);

    row_sum + col_sum       
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
