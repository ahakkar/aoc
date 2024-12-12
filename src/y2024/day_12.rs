/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

#![allow(dead_code)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(clippy::needless_return)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::only_used_in_recursion)]

use crate::{
    util::{self, point},
    Fro, Solution, TaskResult,
};
use grid::*;
use util::point::{Point, ORTHOGONAL};

// Can add more shared vars here
pub struct GardenGroups {
    map: Grid<Plot>,
}

#[derive(Debug, Clone)]
struct Plot {
    ptype: char,
    visited: bool,
    region_id: usize,
}

impl Plot {
    pub const fn new(ptype: char, visited: bool, region_id: usize) -> Plot {
        Plot {
            ptype,
            visited,
            region_id,
        }
    }
}

#[derive(Debug, Clone)]
struct Area {
    p_len: usize,
    size: usize,
    region_id: usize,
}

impl Area {
    pub const fn new(region_id: usize) -> Area {
        Area {
            p_len: 0,
            size: 0,
            region_id,
        }
    }
}

// Can be used to implement fancier task-specific parsing
impl Fro for GardenGroups {
    fn fro(input: &str) -> Self {
        Self {
            map: Grid::from_vec(
                input
                    .replace('\n', "")
                    .chars()
                    .map(|c| Plot::new(c, false, usize::MAX))
                    .collect(),
                (input.len() as f64).sqrt() as usize,
            ),
        }
    }
}

// Main solvers
impl Solution for GardenGroups {
    fn silver(&self) -> TaskResult {
        // Each new unvisited plot is assigned a new unique id as an index
        // Area's perimeter length is the value
        let mut results: Vec<Area> = Vec::new();
        let mut map = self.map.clone();
        let (rows, cols) = map.size();
        let mut region_id = 0;
        let mut visited_regions: Vec<usize> = Vec::new();

        // Iterate through the map and flood fill each unvisited plot
        // Calculate perimeter length on the same go
        for y in 0..rows {
            for x in 0..cols {
                if let Some(l) = map.get(y, x) {
                    if !l.visited && !visited_regions.contains(&l.region_id) {
                        let mut area = Area::new(region_id);
                        Self::flood_fill(
                            self,
                            Point::new(x as i32, y as i32),
                            l.ptype,
                            &mut area,
                            &mut map,
                        );
                        region_id += 1;
                        visited_regions.push(region_id);
                        results.push(area);
                    }
                }
            }
        }

        results.iter().for_each(|r| println!("{:?}", r));
        println!("{:?}", map);
        println!("end!");

        TaskResult::Usize(results.iter().map(|r| r.p_len * r.size).sum())
    }

    fn gold(&self) -> TaskResult {
        TaskResult::String("plaa".to_string())
    }
}

// For assisting functions
impl GardenGroups {
    // Visits connected plots and return the area's perimeter len
    fn flood_fill(
        &self,
        cur: Point,
        ptype: char,
        result: &mut Area,
        map: &mut Grid<Plot>,
    ) {
        // Mark current as visited
        if let Some(current) = Self::get_value(map, &cur.x, &cur.y) {
            if !current.visited {
                current.visited = true;
                current.region_id = result.region_id;
                if current.ptype == ptype {
                    result.size += 1;
                }

                for dir in ORTHOGONAL {
                    let dx = cur.x + dir.x;
                    let dy = cur.y + dir.y;

                    if let Some(neighbor) = Self::get_value(map, &dx, &dy) {
                        if neighbor.ptype == ptype {
                            if !neighbor.visited {
                                Self::flood_fill(
                                    self,
                                    Point::new(dx, dy),
                                    ptype,
                                    result,
                                    map,
                                );
                            }
                        } else {
                            // Was a different type
                            result.p_len += 1;
                        }
                    } else {
                        // Was a map border
                        result.p_len += 1;
                    }
                } // end for
            } // !end current.visited
        }
    }

    // Workaround for Grid crate's broken coordinate order
    fn get_value<'a>(map: &'a mut Grid<Plot>, x: &i32, y: &i32) -> Option<&'a mut Plot> {
        map.get_mut(*y, *x)
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/test/12.txt");
        let queue = GardenGroups::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(1930));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/real/12.txt");
        let queue = GardenGroups::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(0));
        assert_eq!(queue.gold(), TaskResult::Usize(0));
    }
}
