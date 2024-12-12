/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use crate::{
    util::{
        self,
        grid::XyGrid,
        point::{EAST, NORTH, NORTHEAST},
    },
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
    sides: usize,
}

impl Area {
    pub const fn new(region_id: usize) -> Area {
        Area {
            p_len: 0,
            size: 0,
            region_id,
            sides: 0,
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
        let (results, _) = Self::process_regions(self);
        TaskResult::Usize(results.iter().map(|r| r.p_len * r.size).sum())
    }

    fn gold(&self) -> TaskResult {
        let (mut results, mut map) = Self::process_regions(self);
        let (rows, cols) = map.size();

        // Calculate the amount of distinct continous edges for each region
        // by checking north edges and rotating the map 3 times to effectively
        // check all 4 edges.
        for i in 0..4 {
            for y in 0..rows {
                for x in 0..cols {
                    if let Some(current) = map.get(y, x) {
                        let point = Point::new(x as i32, y as i32);

                        // IF we don't have a north edge, continue
                        if let Some(n) = map.get_point(point + NORTH) {
                            if n.region_id == current.region_id {
                                continue;
                            }
                        }

                        // Based on e and ne cells decide if edge continues
                        if map
                            .get_point(point + EAST)
                            .map_or(true, |e| e.region_id != current.region_id)
                            || map
                                .get_point(point + NORTHEAST)
                                .map_or(false, |ne| ne.region_id == current.region_id)
                        {
                            results[current.region_id].sides += 1;
                        }
                    }
                }
            }
            if i < 3 {
                map.rotate_right()
            }
        }

        TaskResult::Usize(results.iter().map(|r| r.sides * r.size).sum())
    }
}

// For assisting functions
impl GardenGroups {
    fn process_regions(&self) -> (Vec<Area>, Grid<Plot>) {
        // Each new unvisited plot is assigned a new unique id as an index
        // Area's perimeter length is the value
        let mut results: Vec<Area> = Vec::new();
        let mut map = self.map.clone();
        let (rows, cols) = map.size();
        let mut region_id = 0;
        let mut visited_flags = Vec::with_capacity(1024); // data had 604 regions

        // Iterate through the map and flood fill each unvisited plot
        // Calculate perimeter length on the same go
        for y in 0..rows {
            for x in 0..cols {
                if let Some(l) = map.get(y, x) {
                    if !l.visited && !visited_flags.get(l.region_id).unwrap_or(&false) {
                        let start = Point::new(x as i32, y as i32);
                        let mut area = Area::new(region_id);

                        Self::flood_fill(start, l.ptype, &mut area, &mut map);
                        region_id += 1;
                        results.push(area);
                        visited_flags.push(true);
                    }
                }
            }
        }
        (results, map)
    }

    // Visits connected plots and return the area's perimeter len
    fn flood_fill(cur: Point, ptype: char, result: &mut Area, map: &mut Grid<Plot>) {
        // Mark current as visited
        if let Some(cur_plot) = map.get_xy_mut(&cur.x, &cur.y) {
            match cur_plot.visited {
                true => return,
                false => cur_plot.visited = true,
            }

            if cur_plot.ptype == ptype {
                cur_plot.region_id = result.region_id;
                result.size += 1;
            }

            for dir in ORTHOGONAL {
                let neighbor_point = Point::new(cur.x + dir.x, cur.y + dir.y);

                match map.get_point(neighbor_point) {
                    Some(neighbor) if neighbor.ptype == ptype => {
                        if !neighbor.visited {
                            Self::flood_fill(neighbor_point, ptype, result, map);
                        }
                    }
                    _ => {
                        // Either out of bounds or a different plot type
                        result.p_len += 1;
                    }
                }
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
        let test_data = read_data_from_file("input/2024/test/12.txt");
        let queue = GardenGroups::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(1930));
        assert_eq!(queue.gold(), TaskResult::Usize(1206));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2024/real/12.txt");
        let queue = GardenGroups::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(1352976));
        assert_eq!(queue.gold(), TaskResult::Usize(808796));
    }
}
