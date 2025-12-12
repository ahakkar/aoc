/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

#![allow(unused_mut)]
#![allow(dead_code)]

use crate::{
    Fro, Solution, TaskResult,
    util::{direction::Direction, point2::Point2},
};
use grid::*;
use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Empty,
    Wall,
    Guard,
    Invalid,
    Visited,
}

// Can add more shared vars here
pub struct GuardGallivant {
    map: Grid<Tile>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for GuardGallivant {
    fn fro(input: &str) -> Self {
        Self {
            map: Grid::from_vec(
                input
                    .replace('\n', "")
                    .chars()
                    .map(|c| Self::match_char(&c))
                    .collect(),
                (input.len() as f64).sqrt() as usize,
            ),
        }
    }
}

// Main solvers
impl Solution for GuardGallivant {
    fn silver(&self) -> TaskResult {
        let mut dir = Direction::North; // 67,89
        let mut pos: Option<Point2> = Some(Point2::new(67, 89)); // 4,6 test
        let mut visited: HashSet<Point2> = HashSet::new();

        while let Some(cur_pos) = pos {
            let dir_vec = dir.to_vector();
            let new_x = cur_pos.x + dir_vec.x;
            let new_y = cur_pos.y + dir_vec.y;

            if let Some(new_pos) = self.map.get(new_y, new_x) {
                // If tile is clear, move to it
                if *new_pos != Tile::Wall {
                    visited.insert(pos.take().unwrap());
                    pos = Some(Point2::new(new_x, new_y));
                }
                // If tile is occupied, turn right and try again
                else {
                    dir = Direction::turn_90(dir, 'r');
                }
            } else {
                break;
            }
        }

        /*         let mut map = self.map.clone();
        println!("visited [{}]: {:?}", visited.len(), visited);

        for v in &visited {
            map[(v.y as usize, v.x as usize)] = Tile::Visited;
        }
        Self::print_grid(map); */

        TaskResult::Usize(visited.len() + 1)
    }

    fn gold(&self) -> TaskResult {
        let start_pos = Point2::new(67, 89); // 67,89 || 4,6
        let total_rows = self.map.rows();
        let total_cols = self.map.cols();

        let loop_positions: usize = (0..total_rows)
            .into_par_iter()
            .map(|row| {
                let mut local_loop_positions = 0;

                for col in 0..total_cols {
                    // Reset state for each column
                    let mut dir = Direction::North;
                    let mut pos = Some(start_pos);
                    let mut visited: HashSet<(Point2, Direction)> = HashSet::new();

                    while let Some(cur_pos) = pos {
                        // Loop found if pos&dir were already in the set
                        if !visited.insert((cur_pos, dir)) {
                            local_loop_positions += 1;
                            break;
                        }

                        let point2 = dir.to_vector();
                        let new_x = cur_pos.x + point2.x;
                        let new_y = cur_pos.y + point2.y;

                        if let Some(new_tile) = self.map.get(new_y, new_x) {
                            // Obstacle?
                            if *new_tile == Tile::Wall
                                || (new_x == col as i64 && new_y == row as i64)
                            {
                                dir = Direction::turn_90(dir, 'r');
                            } else {
                                // Otherwise proceed
                                pos = Some(Point2::new(new_x, new_y));
                            }
                        } else {
                            // Out of bounds
                            break;
                        }
                    }
                }
                local_loop_positions
            })
            .sum::<usize>();

        TaskResult::Usize(loop_positions)
    }
}

// For assisting functions
impl GuardGallivant {
    fn match_char(c: &char) -> Tile {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            '^' => Tile::Guard,
            _ => Tile::Invalid,
        }
    }

    fn print_grid(map: Grid<Tile>) {
        println!(" 0123456789");
        for (i, row) in map.iter_rows().enumerate() {
            print!("{}", i);
            for tile in row {
                print!(
                    "{}",
                    match tile {
                        Tile::Empty => ".",
                        Tile::Wall => "#",
                        Tile::Guard => "^",
                        Tile::Visited => "X",
                        _ => "",
                    }
                );
            }
            println!();
        }
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TaskResult, util::utils::read_data_from_file};

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2024/test/06.txt");
        let queue = GuardGallivant::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(41));
        assert_eq!(queue.gold(), TaskResult::Usize(6));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2024/real/06.txt");
        let queue = GuardGallivant::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(4696));
        assert_eq!(queue.gold(), TaskResult::Usize(1443));
    }
}
