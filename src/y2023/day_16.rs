/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use std::collections::HashSet;

use crate::{
    Fro, Solution, TaskResult,
    util::{
        direction::Direction,
        gridmap::{GridMap, data_as_chars},
        point2::Point2,
        utils::Visited,
    },
};

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
        let map: GridMap<char> = GridMap::new(data_as_chars(&self.data));
        TheFloorWillBeLava::get_energized(&map, Point2::new(-1, 0), Direction::East)
            .into()
    }

    fn gold(&self) -> TaskResult {
        let map: GridMap<char> = GridMap::new(data_as_chars(&self.data));
        let width = map.get_width() as i64;
        let height = map.get_height() as i64;

        let top_row = (0..width).map(|x| (Point2::new(x, -1), Direction::South));
        let bottom_row = (0..width).map(|x| (Point2::new(x, height), Direction::North));
        let left_column = (0..height).map(|y| (Point2::new(-1, y), Direction::East));
        let right_column = (0..height).map(|y| (Point2::new(width, y), Direction::West));

        top_row
            .chain(bottom_row)
            .chain(left_column)
            .chain(right_column)
            .map(|(coord, dir)| TheFloorWillBeLava::get_energized(&map, coord, dir))
            .max()
            .unwrap()
            .into()
    }
}

// For assisting functions
impl TheFloorWillBeLava {
    fn get_energized(map: &GridMap<char>, start: Point2, dir: Direction) -> usize {
        let mut visited: Visited = HashSet::new();
        TheFloorWillBeLava::follow_path(map, start, dir, &mut visited);

        let mut energized: HashSet<Point2> = HashSet::new();
        for visit in visited {
            energized.insert(visit.0);
        }
        energized.len()
    }

    fn follow_path(
        map: &GridMap<char>,
        prev: Point2,
        dir: Direction,
        visited: &mut Visited,
    ) {
        let cur = prev.step(dir);

        if let Some(char) = map.get_cell(&cur) {
            if !visited.insert((cur, dir)) {
                return;
            }

            match char {
                '|' => {
                    if matches!(dir, Direction::East | Direction::West) {
                        TheFloorWillBeLava::follow_path(
                            map,
                            cur,
                            Direction::North,
                            visited,
                        );
                        TheFloorWillBeLava::follow_path(
                            map,
                            cur,
                            Direction::South,
                            visited,
                        );
                    } else {
                        TheFloorWillBeLava::follow_path(map, cur, dir, visited);
                    }
                }
                '-' => {
                    if matches!(dir, Direction::North | Direction::South) {
                        TheFloorWillBeLava::follow_path(
                            map,
                            cur,
                            Direction::East,
                            visited,
                        );
                        TheFloorWillBeLava::follow_path(
                            map,
                            cur,
                            Direction::West,
                            visited,
                        );
                    } else {
                        TheFloorWillBeLava::follow_path(map, cur, dir, visited);
                    };
                }
                '/' => {
                    match dir {
                        Direction::North => TheFloorWillBeLava::follow_path(
                            map,
                            cur,
                            Direction::East,
                            visited,
                        ),
                        Direction::South => TheFloorWillBeLava::follow_path(
                            map,
                            cur,
                            Direction::West,
                            visited,
                        ),
                        Direction::East => TheFloorWillBeLava::follow_path(
                            map,
                            cur,
                            Direction::North,
                            visited,
                        ),
                        Direction::West => TheFloorWillBeLava::follow_path(
                            map,
                            cur,
                            Direction::South,
                            visited,
                        ),
                        _ => panic!("invalid dir"),
                    };
                }
                '\\' => {
                    match dir {
                        Direction::North => TheFloorWillBeLava::follow_path(
                            map,
                            cur,
                            Direction::West,
                            visited,
                        ),
                        Direction::South => TheFloorWillBeLava::follow_path(
                            map,
                            cur,
                            Direction::East,
                            visited,
                        ),
                        Direction::East => TheFloorWillBeLava::follow_path(
                            map,
                            cur,
                            Direction::South,
                            visited,
                        ),
                        Direction::West => TheFloorWillBeLava::follow_path(
                            map,
                            cur,
                            Direction::North,
                            visited,
                        ),
                        _ => panic!("invalid dir"),
                    };
                }
                '.' => TheFloorWillBeLava::follow_path(map, cur, dir, visited), // continue ahead
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

        assert_eq!(queue.silver(), TaskResult::Usize(46));
        assert_eq!(queue.gold(), TaskResult::Usize(51));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2023/real/16.txt");
        let queue = TheFloorWillBeLava::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(7210));
        assert_eq!(queue.gold(), TaskResult::Usize(7673));
    }
}
