/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use std::collections::HashSet;

use crate::{Fro, Solution, TaskResult};

// Can add more shared vars here
pub struct TheFloorWillBeLava {
    data: Vec<String>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for TheFloorWillBeLava {
    fn fro(input: &str) -> Self {
        Self {
            data: input.split('\n').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for TheFloorWillBeLava {
    fn silver(&self) -> TaskResult {
        let map: GridMap<char> = GridMap::new(data_as_chars(data));
        get_energized(&map, Coord::new(-1, 0), EAST).into()
    }

    fn gold(&self) -> TaskResult {
        let map: GridMap<char> = GridMap::new(data_as_chars(data));
        let width = map.get_width() as isize;
        let height = map.get_height() as isize;

        let top_row = (0..width).map(|x| (Coord::new(x, -1), SOUTH));
        let bottom_row = (0..width).map(|x| (Coord::new(x, height), NORTH));
        let left_column = (0..height).map(|y| (Coord::new(-1, y), EAST));
        let right_column = (0..height).map(|y| (Coord::new(width, y), WEST));

        top_row
            .chain(bottom_row)
            .chain(left_column)
            .chain(right_column)
            .map(|(coord, dir)| get_energized(&map, coord, dir))
            .max()
            .unwrap()
            .into()
    }
}

// For assisting functions
impl TheFloorWillBeLava {
    fn get_energized(map: &GridMap<char>, start: Coord, dir: Vec2D) -> usize {
        let mut visited: Visited = HashSet::new();
        follow_path(map, start, dir, &mut visited);

        let mut energized: HashSet<Coord> = HashSet::new();
        for visit in visited {
            energized.insert(visit.0);
        }
        energized.len()
    }

    fn follow_path(map: &GridMap<char>, prev: Coord, dir: Vec2D, visited: &mut Visited) {
        let cur = Coord::new(prev.x + dir.x, prev.y + dir.y);

        if let Some(char) = map.get_cell(&cur) {
            if !visited.insert((cur, dir)) {
                return;
            }

            match char {
                '|' => {
                    if dir == EAST || dir == WEST {
                        follow_path(map, cur, NORTH, visited);
                        follow_path(map, cur, SOUTH, visited);
                    } else {
                        follow_path(map, cur, dir, visited);
                    }
                }
                '-' => {
                    if dir == NORTH || dir == SOUTH {
                        follow_path(map, cur, EAST, visited);
                        follow_path(map, cur, WEST, visited);
                    } else {
                        follow_path(map, cur, dir, visited);
                    };
                }
                '/' => {
                    match dir {
                        NORTH => follow_path(map, cur, EAST, visited),
                        SOUTH => follow_path(map, cur, WEST, visited),
                        EAST => follow_path(map, cur, NORTH, visited),
                        WEST => follow_path(map, cur, SOUTH, visited),
                        _ => panic!("invalid dir"),
                    };
                }
                '\\' => {
                    match dir {
                        NORTH => follow_path(map, cur, WEST, visited),
                        SOUTH => follow_path(map, cur, EAST, visited),
                        EAST => follow_path(map, cur, SOUTH, visited),
                        WEST => follow_path(map, cur, NORTH, visited),
                        _ => panic!("invalid dir"),
                    };
                }
                '.' => follow_path(map, cur, dir, visited), // continue ahead
                _ => panic!("Invalid char at map {:?}, char: {}", cur, char),
            }
        }
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2023/test/16.txt");
        let queue = TheFloorWillBeLava::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2023/real/16.txt");
        let queue = TheFloorWillBeLava::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(7210));
        assert_eq!(queue.gold(), TaskResult::Usize(7673));
    }
}
