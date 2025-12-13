/*
 * 2023 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use pathfinding::{directed::dijkstra::dijkstra, matrix::*};

type Point = (usize, usize);
type Vec2D = (isize, isize); // move_in_direction expects a (isize, isize) tuple

#[derive(Clone, Eq, Hash, PartialEq)]
struct State {
    pos: Point,
    dir: Vec2D,
    length: usize,
}

use crate::{Fro, Solution, TaskResult};

// Can add more shared vars here
pub struct ClumsyCrucible {
    data: Vec<String>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for ClumsyCrucible {
    fn fro(input: &str) -> Self {
        Self {
            data: input.split('\n').map(|line| line.to_string()).collect(),
        }
    }
}

// Main solvers
impl Solution for ClumsyCrucible {
    fn silver(&self) -> TaskResult {
        let mut grid: Matrix<u32> = Matrix::new_empty(data[0].len());
        for row in data {
            let parsed: Vec<u32> = row.chars().map(|c| c.to_digit(10).unwrap()).collect();
            grid.extend(&parsed).unwrap();
        }
        (get_lowest_weight_path(grid, 1, 3) as usize).into()
    }

    fn gold(&self) -> TaskResult {
        let mut grid: Matrix<u32> = Matrix::new_empty(data[0].len());
        for row in data {
            let parsed: Vec<u32> = row.chars().map(|c| c.to_digit(10).unwrap()).collect();
            grid.extend(&parsed).unwrap();
        }
        (get_lowest_weight_path(grid, 4, 10) as usize).into()
    }
}

// For assisting functions
impl ClumsyCrucible {
    // https://docs.rs/pathfinding/latest/pathfinding/directed/dijkstra/fn.dijkstra.html
    // based on https://gist.github.com/samueltardieu/0bf0763fb4b2810ff4a4721c90398450
    fn get_lowest_weight_path(
        grid: Matrix<u32>,
        min_move: usize,
        max_move: usize,
    ) -> u32 {
        type Successors = Box<dyn FnMut(&State) -> Vec<(State, u32)>>;

        // Start from 0,0, not moving, no length
        let start: State = State {
            pos: (0, 0),
            dir: (0, 0),
            length: 0,
        };
        let grid_rows: usize = grid.rows;
        let grid_columns: usize = grid.columns;

        // Dijkstra visits these nodes next
        let successors: Successors = Box::new(move |state| {
            // At most three directions are considered: straight, left or right
            let mut next = Vec::with_capacity(3);

            // Helper function encapsulates the logic for adding new moves to the "next" vector
            let mut add_possible_moves = |dir: Vec2D, length: usize| {
                next.extend(grid.move_in_direction(state.pos, dir).map(|candidate| {
                    (
                        State {
                            pos: candidate,
                            dir,
                            length,
                        },
                        grid[candidate],
                    )
                }));
            };

            // If these conditions match, we add each true condition to the next possible moves
            // 1. If length is less than max_move, we can continue moving in the same direction
            if state.length < max_move {
                add_possible_moves(state.dir, state.length + 1);
            }

            // 2. If length is at least min_move, we can turn left or right from current direction
            if state.length >= min_move {
                add_possible_moves((-state.dir.1, -state.dir.0), 1);
                add_possible_moves((state.dir.1, state.dir.0), 1);
            }
            // 3. If length is 0, we can move down or right
            else if state.length == 0 {
                add_possible_moves((1, 0), 1);
                add_possible_moves((0, 1), 1);
            }

            next
        });

        // Condition: bottom right corner and movement was at least min movement
        let success: Box<dyn Fn(&State) -> bool> = Box::new(move |state| {
            state.pos == (grid_rows - 1, grid_columns - 1) && state.length >= min_move
        });

        let result = dijkstra(&start, successors, success);

        match result {
            Some((_, cost)) => cost,
            None => 0,
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
        let test_data = read_data_from_file("input/2023/test/17.txt");
        let queue = ClumsyCrucible::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(102));
        assert_eq!(queue.gold(), TaskResult::Usize(71));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2023/real/17.txt");
        let queue = ClumsyCrucible::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(814));
        assert_eq!(queue.gold(), TaskResult::Usize(974));
    }
}
