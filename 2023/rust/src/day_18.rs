/*
 * 2023 Advent of Code with Rust
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

use super::utils::*;

pub fn solve(data: Vec<String>) {    
    println!("Silver: {}", silver(&data));
    //println!("Gold: {}", gold(&data));
}

fn flood_fill(grid: &mut Grid<char>, start_x: usize, start_y: usize, fill_char: char) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut stack = vec![(start_x, start_y)];
    let mut count = 0;

    while let Some((x, y)) = stack.pop() {
        if x >= cols || y >= rows || grid[y][x] != '.' {
            continue;
        }

        // Fill the cell and increment the count
        grid[y][x] = fill_char;
        count += 1;

        // Add neighboring cells to the stack
        if x > 0 { stack.push((x - 1, y)); }
        if x + 1 < cols { stack.push((x + 1, y)); }
        if y > 0 { stack.push((x, y - 1)); }
        if y + 1 < rows { stack.push((x, y + 1)); }
    }    

    count
}

fn dig(dpos: &mut Point, grid: &mut Grid<char>, dir: &str, amt: usize) -> Point {
    match dir.chars().next().unwrap() {
        'R' => for i in 0..amt { grid[dpos.1][dpos.0+1] = '#'; dpos.0 += 1; },
        'L' => for i in 0..amt { grid[dpos.1][dpos.0-1] = '#'; dpos.0 -= 1; },
        'U' => for i in 0..amt { grid[dpos.1-1][dpos.0] = '#'; dpos.1 -= 1; },
        'D' => for i in 0..amt { grid[dpos.1+1][dpos.0] = '#'; dpos.1 += 1; },
         _  => panic!("bad dig dir"),
    }
    *dpos
}

fn silver(data: &[String]) -> usize {
    let mut sum: usize = 0;    
    let mut grid: Grid<char> = vec![vec!['.'; 4096]; 4096];
    let mut dpos: Point = (2048,2048);
    grid[2048][2048] = '#'; // digger starts here

    // dig
    for row in data {
        let (dir, amtcolor) = row.trim().split_once(' ').unwrap();
        let (amt, color) = amtcolor.trim().split_once(' ').unwrap();
        sum += amt.parse::<usize>().unwrap();
        dig(
            &mut dpos,
            &mut grid,
            dir, 
            amt.parse::<usize>().unwrap()
        );
        //println!("{} {} {}", dir, amt, color);
    }
    //assert_eq!(sum, 38);
    //print_map(&grid);

    // flood
    sum += flood_fill(&mut grid, 2049, 2049, '#');
    //assert_eq!(sum, 62);
    //print_map(&grid);
    
    sum // 50465
}

/* fn gold(data: &Vec<String>) -> usize {
    let mut sum: usize = 0;    

    for row in data {
           } 
    sum 
} */

// run these with cargo test --bin main -- day_XX::tests
#[cfg(test)]
mod tests {
    use crate::utils::read_data_from_file;
    use super::*;   

    #[test]
    fn test_test() {
        let test_data:Vec<String> = read_data_from_file("input/test/18.txt");
        assert_eq!(silver(&test_data), 62);
        //assert_eq!(gold(&test_data), 145);
    }

    #[test]
    fn test_silver() {
        let test_data:Vec<String> = read_data_from_file("input/real/18.txt");
        assert_eq!(silver(&test_data), 50465);
    }

    #[test]
    fn test_gold() {
        let test_data:Vec<String> = read_data_from_file("input/real/18.txt");
        //assert_eq!(gold(&test_data), 212763);
    }
}