/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use crate::{Fro, Solution, TaskResult, util::point2::Point2};

// Can add more shared vars here
pub struct LavaductLagoon {
    data: Vec<String>,
}

type Grid<T> = Vec<Vec<T>>;

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
        let mut dpos = Point2::new(2048, 2048);
        grid[2048][2048] = '#'; // digger starts here

        // dig
        for row in &self.data {
            let (dir, amtcolor) = row.trim().split_once(' ').unwrap();
            let (amt, _) = amtcolor.trim().split_once(' ').unwrap();
            sum += amt.parse::<usize>().unwrap();
            LavaductLagoon::dig(&mut dpos, &mut grid, dir, amt.parse::<usize>().unwrap());
        }
        (sum + LavaductLagoon::flood_fill(&mut grid, 2049, 2049, '#')).into()
    }

    fn gold(&self) -> TaskResult {
        let mut sum: isize = 0;
        let mut bsum: isize = 0;
        let mut dpos = Point2::new(0, 0);
        let mut vertices: Vec<Point2> = vec![];
        vertices.push(Point2::new(0, 0)); // start

        // parse to vertices
        for row in &self.data {
            let (_, amtcolor) = row.trim().split_once(' ').unwrap();
            let (_, mut color) = amtcolor.trim().split_once(' ').unwrap();

            color = color.trim_start_matches("(#").trim_end_matches(')');
            let dir = u8::from_str_radix(&color[5..6], 16).unwrap();
            let length = isize::from_str_radix(&color[0..5], 16).unwrap();
            bsum += length;
            LavaductLagoon::add_vertex(&mut dpos, &mut vertices, dir, length as i64);
        }

        // use shoelace formula to calculate area
        // https://en.wikipedia.org/wiki/Shoelace_formula
        let n = vertices.len();
        for i in 0..n {
            let x1 = vertices[i].x;
            let y1 = vertices[i].y;

            let x2 = vertices[(i + 1) % n].x;
            let y2 = vertices[(i + 1) % n].y;

            sum += (x1 * y2 - y1 * x2) as isize;
        }
        (((sum + bsum) / 2 + 1) as usize).into()
    }
}

// For assisting functions
impl LavaductLagoon {
    fn add_vertex(dpos: &mut Point2, vertices: &mut Vec<Point2>, dir: u8, length: i64) {
        match dir {
            /*R */
            0 => {
                vertices.push(Point2::new(dpos.x + length, dpos.y));
                dpos.x += length
            }
            /*D */
            1 => {
                vertices.push(Point2::new(dpos.x, dpos.y + length));
                dpos.y += length
            }
            /*L */
            2 => {
                vertices.push(Point2::new(dpos.x - length, dpos.y));
                dpos.x -= length
            }
            /*U */
            3 => {
                vertices.push(Point2::new(dpos.x, dpos.y - length));
                dpos.y -= length
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

    fn dig(dpos: &mut Point2, grid: &mut Grid<char>, dir: &str, amt: usize) -> Point2 {
        match dir.chars().next().unwrap() {
            'R' => {
                for _ in 0..amt {
                    grid[dpos.y as usize][(dpos.x + 1) as usize] = '#';
                    dpos.x += 1;
                }
            }
            'L' => {
                for _ in 0..amt {
                    grid[dpos.y as usize][(dpos.x - 1) as usize] = '#';
                    dpos.x -= 1;
                }
            }
            'U' => {
                for _ in 0..amt {
                    grid[(dpos.y - 1) as usize][dpos.x as usize] = '#';
                    dpos.y -= 1;
                }
            }
            'D' => {
                for _ in 0..amt {
                    grid[(dpos.y + 1) as usize][dpos.x as usize] = '#';
                    dpos.y += 1;
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
