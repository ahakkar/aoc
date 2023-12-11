/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_range_loop)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use std::fmt;
// - parse # coordinates to a hashmap for example
// - while parsing add empty rows and columns to a list
// - transform coordinates based on empty rows and columns
// - collect pairs from each coordinate, amount of pairs should be:
//   bionmial coefficient..
// - and a taxicab (manhattan) distance to calculate distances between
//   coordinates
// - finally calculate each pairs taxicab distance, ideally doing this
//   during pair forming ofc.. maybe pairs could be structs which hold info
//   which might be needed later. sum all distances as answer

struct Coord {
    x: usize,
    y: usize,
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}
    

fn binomial_coefficient(n: usize, k: usize) -> usize {
    let mut result = 1;
    for i in 0..k {
        result *= n - i;
        result /= i + 1;
    }
    result
}

fn manhattan_distance(a: &Coord, b: &Coord) -> i64 {
    (
        (b.x as i64 - a.x as i64).abs() + 
        (b.y as i64 - a.y as i64).abs()
    )
}

fn data_as_chars(data: &[String]) -> Vec<Vec<char>> {
    let mut data_as_chars: Vec<Vec<char>> = vec![];
    for row in data {
        data_as_chars.push(row.chars().collect::<Vec<char>>());
    }
    data_as_chars
}

fn count_empty_cols(grid: &[Vec<char>]) -> Vec<usize> {
    let mut empty_cols: Vec<usize> = vec![];
    let num_cols = grid[0].len();

    for col_idx in 0..num_cols {
        if grid.iter().all(|row| row[col_idx] == '.') {
            empty_cols.push(col_idx);
        }
    }
    empty_cols
}

fn transform_coordinates(
    galaxies: &mut [Coord],
    empty_rows: &[usize],
    empty_cols: &[usize],
    step: usize
) {
    for galaxy in galaxies {
        galaxy.y += empty_rows.iter().filter(|&&row| row < galaxy.y).count() * (step-1);
        galaxy.x += empty_cols.iter().filter(|&&col| col < galaxy.x).count() * (step-1);
    }
}

fn calc_pair_dist_sums(galaxies: Vec<Coord>) -> i64 {
    let mut sum: i64 = 0;
    let mut pairs = vec![];

    // Function to generate pairs
    for (i, coord1) in galaxies.iter().enumerate() {
        for coord2 in &galaxies[i + 1..] {
            pairs.push((coord1, coord2));
        }
    }

    pairs.iter().map(|(a, b)| manhattan_distance(a, b)).sum()
}

fn silver(data: &[String]) -> i64 {

    let grid: Vec<Vec<char>> = data_as_chars(data);
    let mut galaxies: Vec<Coord> = vec![];    
    let mut empty_rows: Vec<usize> = vec![];
    let empty_cols: Vec<usize> = count_empty_cols(&grid);
    let mut is_row_empty;

    for (y, row) in grid.iter().enumerate() {
        is_row_empty = true;
        for (x, char) in row.iter().enumerate() {   
            if char == &'#' { 
                is_row_empty = false;
                galaxies.push(Coord{x, y});
            }
        }
        if is_row_empty { empty_rows.push(y) }
    }

    transform_coordinates(&mut galaxies, &empty_rows, &empty_cols, 1);
    calc_pair_dist_sums(galaxies) // 9623138
}

fn gold(data: &[String]) -> i64 {

    let grid: Vec<Vec<char>> = data_as_chars(data);
    let mut galaxies: Vec<Coord> = vec![];    
    let mut empty_rows: Vec<usize> = vec![];
    let empty_cols: Vec<usize> = count_empty_cols(&grid);
    let mut is_row_empty;
    let step = 1_000_000;

    for (y, row) in grid.iter().enumerate() {
        is_row_empty = true;
        for (x, char) in row.iter().enumerate() {   
            if char == &'#' { 
                is_row_empty = false;
                galaxies.push(Coord{x, y});
            }
        }
        if is_row_empty { empty_rows.push(y) }
    }


    transform_coordinates(&mut galaxies, &empty_rows, &empty_cols, step);

    // step 1: 374
    // step 10: 1030
    // step 100: 8410
    calc_pair_dist_sums(galaxies) // 726820896326 too high, 726820169514
}

fn print_map(galaxies: &[Coord], map_w: usize, map_h: usize) {
    let mut map:Vec<Vec<char>> = vec![vec!['.'; map_w]; map_h];
    for galaxy in galaxies {
        map[galaxy.y][galaxy.x] = '#';
    }
    println!("map size: [{} x {}]", map_w, map_h);
    for row in &map {
        let row_string: String = row.iter().collect();
        println!("{}", row_string);
    }
}

pub fn solve(data: Vec<String>) {
    println!("Silver: {}", silver(&data));
    println!("Gold: {}", gold(&data));
}