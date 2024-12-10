/*
 * 2024 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

use crate::{Fro, Solution, TaskResult};
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
struct Block {
    id: usize,
    data: char,
}

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.data == '.' {
            writeln!(f, "Empty data block")
        } else {
            writeln!(f, "{:?}", self)
        }
    }
}

// Can add more shared vars here
pub struct DiskFragmenter {
    data: Vec<Block>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for DiskFragmenter {
    fn fro(input: &str) -> Self {
        let mut data: Vec<Block> = Vec::new();
        let mut i = 0;
        let mut data_idx = 0;
        input.chars().for_each(|c| {
            let len = c as u8 - 48;
            let mut data_added = false;
            for _ in 0..len {
                if i % 2 == 0 {
                    data.push(Block {
                        id: data_idx,
                        data: c,
                    });
                    data_added = true;
                } else {
                    data.push(Block { id: i, data: '.' });
                }
            }
            if data_added {
                data_idx += 1;
            }
            i += 1;
        });
        Self { data }
    }
}

// Main solvers
impl Solution for DiskFragmenter {
    fn silver(&self) -> TaskResult {
        let mut defrag = self.data.clone();

        // first find the next free space, and hold it in index
        let mut next_free = 0;
        let mut next_end = defrag.len() - 1;

        while let Some(idx) = Self::next_free_idx(&defrag, next_free) {
            // Find next data block from end
            if let Some(end) = Self::next_end_idx(&defrag, next_end) {
                if idx >= end {
                    break;
                } // Finish condition

                // Swap the free space with data from the end
                defrag.swap(idx, end);
                next_free = idx + 1;
                next_end = end - 1;
            } else {
                break;
            } // No more data blocks to process
        }
        TaskResult::Usize(Self::checksum(defrag))
    }

    fn gold(&self) -> TaskResult {
        let mut defrag = self.data.clone();
        let mut seek_end = defrag.len() - 1;

        // Start processing a file
        while let Some(end) = Self::next_end_idx(&defrag, seek_end) {
            if 0 == end {
                break;
            }

            // Skip empty blocks
            if defrag[end].data == '.' {
                seek_end = end.saturating_sub(1);
                continue;
            }

            // Found a file
            let file_length = defrag[end].data.to_digit(10).unwrap() as usize;
            let file_start = end + 1 - file_length;

            // Find room for file
            if let Some(free_range) = Self::free_block(&defrag, &0, &file_start, &file_length) {
                let file_start = end + 1 - file_length;
                Self::move_file(&mut defrag, &free_range, &file_start, &file_length);
            }
            seek_end = file_start.saturating_sub(1);
        }
        TaskResult::Usize(Self::checksum(defrag))
    }
}

// For assisting functions
impl DiskFragmenter {
    // cargo test --lib tests::free_block -- --nocapture
    fn free_block(
        defrag: &[Block],
        start: &usize,
        end: &usize,
        seek_len: &usize,
    ) -> Option<(usize, usize)> {
        let mut continuous = false;
        let mut found = 0;

        #[allow(clippy::needless_range_loop)]
        for offset in *start..=*end {
            if defrag[offset].data == '.' {
                if !continuous {
                    continuous = true
                };
                found += 1;

                if found == *seek_len {
                    return Some((offset + 1 - seek_len, offset));
                }
                continue;
            }
            // Reset state
            continuous = false;
            found = 0;
        }
        None
    }

    fn move_file(
        defrag: &mut [Block],
        free_block: &(usize, usize),
        file_start: &usize,
        file_length: &usize,
    ) -> bool {
        //println!("{:?}, fe:{}, fl:{}", free_block, file_start, file_length);
        (0..*file_length).for_each(|i| {
            defrag.swap(free_block.0 + i, file_start + i);
        });

        true
    }

    fn next_free_idx(defrag: &[Block], start: usize) -> Option<usize> {
        (start..defrag.len()).find(|&i| defrag[i].data == '.')
    }

    fn next_end_idx(defrag: &[Block], end: usize) -> Option<usize> {
        (0..=end).rev().find(|&i| defrag[i].data != '.')
    }

    fn checksum(defrag: Vec<Block>) -> usize {
        defrag
            .iter()
            .enumerate()
            .map(|(i, val)| if val.data != '.' { val.id * i } else { 0 })
            .sum()
    }
}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_data_from_file;

    #[test]
    fn free_block() {
        //let test_data = read_data_from_file("input/test/09.txt");
        //let prog = DiskFragmenter::fro(&test_data);
        let defrag: Vec<Block> = Vec::from([
            Block { id: 0, data: '1' },
            Block { id: 1, data: '.' },
            Block { id: 2, data: '.' },
            Block { id: 3, data: '.' },
            Block { id: 4, data: '2' },
            Block { id: 5, data: '2' },
            Block { id: 6, data: '.' },
        ]);
        assert_eq!(
            DiskFragmenter::free_block(&defrag, &0, &6, &1),
            Some((1, 1))
        );
        assert_eq!(
            DiskFragmenter::free_block(&defrag, &0, &6, &2),
            Some((1, 2))
        );
        assert_eq!(
            DiskFragmenter::free_block(&defrag, &2, &6, &2),
            Some((2, 3))
        );
        assert_eq!(
            DiskFragmenter::free_block(&defrag, &1, &6, &3),
            Some((1, 3))
        );
        assert_eq!(
            DiskFragmenter::free_block(&defrag, &3, &6, &1),
            Some((3, 3))
        );
        assert_eq!(
            DiskFragmenter::free_block(&defrag, &4, &6, &1),
            Some((6, 6))
        );
    }

    #[test]
    fn move_file() {
        let mut defrag: Vec<Block> = Vec::from([
            Block { id: 0, data: '1' },
            Block { id: 1, data: '.' },
            Block { id: 2, data: '.' },
            Block { id: 3, data: '.' },
            Block { id: 4, data: '2' },
            Block { id: 5, data: '2' },
            Block { id: 6, data: '1' },
        ]);

        if let Some(free_block) = DiskFragmenter::free_block(&defrag, &0, &(6 - 2), &2) {
            assert!(DiskFragmenter::move_file(&mut defrag, &free_block, &6, &1));
        }

        //println!("\nstate after swap 1:");
        defrag.iter().for_each(|d| print!("{}", d));

        if let Some(free_block) = DiskFragmenter::free_block(&defrag, &0, &(6 - 2), &2) {
            assert!(DiskFragmenter::move_file(&mut defrag, &free_block, &4, &2));
        }

        //println!("\nstate after swap 2:");
        defrag.iter().for_each(|d| print!("{}", d));

        defrag.push(Block { id: 7, data: '4' });
        defrag.push(Block { id: 8, data: '4' });
        defrag.push(Block { id: 9, data: '4' });
        defrag.push(Block { id: 10, data: '4' });
        defrag.push(Block { id: 11, data: '.' });
        defrag.push(Block { id: 12, data: '3' });
        defrag.push(Block { id: 13, data: '3' });
        defrag.push(Block { id: 14, data: '3' });

        if let Some(free_block) = DiskFragmenter::free_block(&defrag, &0, &(14 - 3), &3) {
            assert!(DiskFragmenter::move_file(&mut defrag, &free_block, &12, &3));
        }

        //println!("\nstate after swap 3:");
        defrag.iter().for_each(|d| print!("{}", d));
    }

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/test/09.txt");
        let queue = DiskFragmenter::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(1928));
        assert_eq!(queue.gold(), TaskResult::Usize(2858));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/real/09.txt");
        let queue = DiskFragmenter::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(6242766523059));
        assert_eq!(queue.gold(), TaskResult::Usize(6272188244509));
    }
}
