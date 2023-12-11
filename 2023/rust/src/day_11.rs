/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
 */

use super::utils::{Coord, manhattan_distance, data_as_chars};

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
    mut galaxies: Vec<Coord>,
    empty_rows: &[usize],
    empty_cols: &[usize],
    step: usize
) -> Vec<Coord> {
    for galaxy in galaxies.iter_mut() {
        galaxy.y += empty_rows.iter().filter(|&&row| row < galaxy.y).count() * (step-1);
        galaxy.x += empty_cols.iter().filter(|&&col| col < galaxy.x).count() * (step-1);
    }
    galaxies.clone()
}

fn calc_pair_dist_sums(galaxies: Vec<Coord>) -> i64 {
    let mut pairs = vec![];

    // Function to generate pairs
    for (i, coord1) in galaxies.iter().enumerate() {
        for coord2 in &galaxies[i + 1..] {
            pairs.push((coord1, coord2));
        }
    }

    pairs.iter().map(|(a, b)| manhattan_distance(a, b)).sum()
}

pub fn solve(data: Vec<String>) {
    let grid: Vec<Vec<char>> = data_as_chars(&data);
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
    let silver = calc_pair_dist_sums(
        transform_coordinates(galaxies.clone(), &empty_rows, &empty_cols, 2)
    );
    
    let gold = calc_pair_dist_sums(
        transform_coordinates(galaxies.clone(), &empty_rows, &empty_cols, 1_000_000)
    );

    println!("Silver: {}", silver); // 9623138 
    println!("Gold: {}", gold);     // 726820896326 too high, 726820169514
}