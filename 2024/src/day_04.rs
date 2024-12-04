/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_range_loop)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use regex::Regex;

use super::utils::*;

fn get_char(data: &[String], y: usize, x: usize) -> char {
    data
        .get(y)
        .and_then(|row| row.get(x..x+1))
        .unwrap()
        .chars()
        .next()
        .unwrap()
}

fn flatten_data(data: &[String]) -> (String, String, String) {
    let mut vert = String::new();
    let mut r_dg = String::new();
    let mut l_dg = String::new();
    let (rows, cols) = (data.len(), data[0].len());

    for x in 0..cols {
        for y in 0..rows {
            vert.push(get_char(data, y, x));
        }
        vert.push(' ');
    }

    // Right diagonal
    for col in 0..cols {
        let mut x = col;
        let mut y = 0;
        while x < cols && y < rows {
            r_dg.push(get_char(data, y, x));
            x += 1;
            y += 1;
        }
        r_dg.push(' ');
    }
    for row in 1..rows {
        let mut x = 0;
        let mut y = row;
        while x < cols && y < rows {
            r_dg.push(get_char(data, y, x));
            x += 1;
            y += 1;
        }
        r_dg.push(' ');
    }


    // Left diagonal
    for col in (0..cols).rev() {
        let mut x = col;
        let mut y = 0;
        while x < cols && y < rows {
            l_dg.push(get_char(data, y, x));
            if x == 0 { break; }
            x -= 1;
            y += 1;
        }
        l_dg.push(' ');
    }
    for row in 1..rows {
        let mut x = cols - 1;
        let mut y = row;
        while x < cols && y < rows {
            l_dg.push(get_char(data, y, x));
            if x == 0 { break; }
            x -= 1;
            y += 1;
        }
        l_dg.push(' ');
    }

    (vert, r_dg, l_dg)
}

pub fn silver(data: &[String]) -> usize {
    let f = Regex::new(r"XMAS").unwrap();
    let b = Regex::new(r"SAMX").unwrap();
    let mut sum: usize = 0;    
    let horz = data.join(" ");
    let (vert, dg_r, dg_l) = flatten_data(data);

    sum += f.find_iter(&horz).count();
    sum += b.find_iter(&horz).count();    
    sum += f.find_iter(&vert).count();
    sum += b.find_iter(&vert).count();   
    sum += f.find_iter(&dg_r).count();
    sum += b.find_iter(&dg_r).count();  
    sum += f.find_iter(&dg_l).count();
    sum += b.find_iter(&dg_l).count();  

    sum
}

pub fn gold(data: &[String]) -> usize {
    let mut sum: usize = 0;    
    let map:GridMap<char> = GridMap::new(data_as_chars(data)); 

    for y in 1..(data.len() - 1) {
        for x in 1..(data[0].len() - 1) {
            if let Some('A') = map.get_char(x, y) {    
                let top_left     = map.get_char(x  - 1, y  - 1);
                let bottom_right = map.get_char(x  + 1, y  + 1);
                let top_right    = map.get_char(x  + 1, y  - 1);
                let bottom_left  = map.get_char(x  - 1, y  + 1);

                if (top_left == Some(&'M') && bottom_right == Some(&'S') ||
                    top_left == Some(&'S') && bottom_right == Some(&'M'))
                    && 
                   (top_right == Some(&'M') && bottom_left == Some(&'S') ||
                    top_right == Some(&'S') && bottom_left == Some(&'M'))
                {
                    sum += 1;
                }                
            }
        }
    }
    sum
}

// cargo test --bin main -- day_XX::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;
    use super::*;   

    #[test]
    fn test_test() {
        let test_data:Vec<String> = read_data_from_file("input/test/04.txt");
        assert_eq!(silver(&test_data), 18);
        assert_eq!(gold(&test_data), 9);
    }

    #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/04.txt");
        assert_eq!(silver(&test_data), 2718);
    }

    #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/04.txt");
        assert_eq!(gold(&test_data), 2046);
    }
}