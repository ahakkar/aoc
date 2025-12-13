/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use crate::{Fro, Solution, TaskResult};

// Can add more shared vars here
pub struct LavaductLagoon {
    data: Vec<String>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for LavaductLagoon {
    fn fro(input: &str) -> Self {
        Self {
            data: input.split('\n').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for LavaductLagoon {
    fn silver(&self) -> TaskResult {
        let mut sum: usize = 0;
        let mut grid: Grid<char> = vec![vec!['.'; 4096]; 4096];
        let mut dpos: Point = (2048, 2048);
        grid[2048][2048] = '#'; // digger starts here

        // dig
        for row in data {
            let (dir, amtcolor) = row.trim().split_once(' ').unwrap();
            let (amt, _) = amtcolor.trim().split_once(' ').unwrap();
            sum += amt.parse::<usize>().unwrap();
            dig(&mut dpos, &mut grid, dir, amt.parse::<usize>().unwrap());
        }
        (sum + flood_fill(&mut grid, 2049, 2049, '#')).into()
    }

    fn gold(&self) -> TaskResult {
        let mut sum: isize = 0;
        let mut bsum: isize = 0;
        let mut dpos: PointI = (0, 0);
        let mut vertices: Vec<PointI> = vec![];
        vertices.push((0, 0)); // start

        // parse to vertices
        for row in data {
            let (_, amtcolor) = row.trim().split_once(' ').unwrap();
            let (_, mut color) = amtcolor.trim().split_once(' ').unwrap();

            color = color.trim_start_matches("(#").trim_end_matches(')');
            let dir = u8::from_str_radix(&color[5..6], 16).unwrap();
            let length = isize::from_str_radix(&color[0..5], 16).unwrap();
            bsum += length;
            add_vertex(&mut dpos, &mut vertices, dir, length);
        }

        // use shoelace formula to calculate area
        // https://en.wikipedia.org/wiki/Shoelace_formula
        let n = vertices.len();
        for i in 0..n {
            let x1 = vertices[i].0;
            let y1 = vertices[i].1;

            let x2 = vertices[(i + 1) % n].0;
            let y2 = vertices[(i + 1) % n].1;

            sum += x1 * y2 - y1 * x2;
        }
        (((sum + bsum) / 2 + 1) as usize).into()
    }
}

// For assisting functions
impl LavaductLagoon {
    fn add_vertex(dpos: &mut PointI, vertices: &mut Vec<PointI>, dir: u8, length: isize) {
        match dir {
            /*R */
            0 => {
                vertices.push((dpos.0 + length, dpos.1));
                dpos.0 += length
            }
            /*D */
            1 => {
                vertices.push((dpos.0, dpos.1 + length));
                dpos.1 += length
            }
            /*L */
            2 => {
                vertices.push((dpos.0 - length, dpos.1));
                dpos.0 -= length
            }
            /*U */
            3 => {
                vertices.push((dpos.0, dpos.1 - length));
                dpos.1 -= length
            }
            _ => panic!("invalid dir num"),
        }
    }

    fn flood_fill(
        grid: &mut Grid<char>,
        start_x: usize,
        start_y: usize,
        fill_char: char,
    ) -> usize {
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
            if x > 0 {
                stack.push((x - 1, y));
            }
            if x + 1 < cols {
                stack.push((x + 1, y));
            }
            if y > 0 {
                stack.push((x, y - 1));
            }
            if y + 1 < rows {
                stack.push((x, y + 1));
            }
        }

        count
    }

    fn dig(dpos: &mut Point, grid: &mut Grid<char>, dir: &str, amt: usize) -> Point {
        match dir.chars().next().unwrap() {
            'R' => {
                for _ in 0..amt {
                    grid[dpos.1][dpos.0 + 1] = '#';
                    dpos.0 += 1;
                }
            }
            'L' => {
                for _ in 0..amt {
                    grid[dpos.1][dpos.0 - 1] = '#';
                    dpos.0 -= 1;
                }
            }
            'U' => {
                for _ in 0..amt {
                    grid[dpos.1 - 1][dpos.0] = '#';
                    dpos.1 -= 1;
                }
            }
            'D' => {
                for _ in 0..amt {
                    grid[dpos.1 + 1][dpos.0] = '#';
                    dpos.1 += 1;
                }
            }
            _ => panic!("bad dig dir"),
        }
        *dpos
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2023/test/18.txt");
        let queue = LavaductLagoon::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(62));
        assert_eq!(queue.gold(), TaskResult::Usize(952408144115));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2023/real/18.txt");
        let queue = LavaductLagoon::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(50465));
        assert_eq!(queue.gold(), TaskResult::Usize(82712746433310));
    }
}
