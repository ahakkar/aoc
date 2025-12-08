/*
 * 2025 Advent of Code with Rust
 * Author: Antti Hakkarainen
 * https://github.com/ahakkar/
**/

#![allow(clippy::needless_range_loop)]

use crate::{
    Fro, Solution, TaskResult,
    util::{dsu::DSU, point3d::Point3d},
};
use std::collections::{BinaryHeap, HashMap};

/*
squared distance between a,b (precise, easy to sort)
an index for Point3d in the data vec
an index for Point3d in the data vec
*/
type HeapItem = (i64, usize, usize);

// Can add more shared vars here
pub struct Playground {
    data: Vec<Point3d>,
}

// Can be used to implement fancier task-specific parsing
impl Fro for Playground {
    fn fro(input: &str) -> Self {
        Self {
            // Discards bad input silently
            data: input
                .split('\n')
                .filter_map(Point3d::new_from_str)
                .collect(),
        }
    }
}

// Main solvers
impl Solution for Playground {
    fn silver(&self) -> TaskResult {
        let mut dsu = DSU::new(self.data.len());
        let mut heap: BinaryHeap<HeapItem> = BinaryHeap::new();
        let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut product: Vec<usize> = vec![];
        let k = 1000; // 10 = test, 1000=real

        // Find k closest connections with brute force
        for i in 0..self.data.len() {
            for j in i + 1..self.data.len() {
                let dist2 = Point3d::squared_distance(self.data[i], self.data[j]);

                if heap.len() < k {
                    heap.push((dist2, i, j));
                } else if let Some(value) = heap.peek()
                    && dist2 < value.0
                {
                    heap.pop();
                    heap.push((dist2, i, j));
                }
            }
        }

        let sorted = heap.into_sorted_vec();

        // Make sets from points with equal distances to each other with DSU
        for (_, a, b) in sorted {
            dsu.union(a, b);
        }

        for i in 0..self.data.len() {
            let root = dsu.find(i);
            groups.entry(root).or_default().push(i);
        }

        // Find and product top 3 set sizes
        let mut items: Vec<(&usize, &Vec<usize>)> = groups.iter().collect();
        items.sort_by_key(|(_, v)| std::cmp::Reverse(v.len()));

        for i in 0..3 {
            product.push(items[i].1.len());
        }

        TaskResult::Usize(product.iter().product())
    }

    fn gold(&self) -> TaskResult {
        let mut dsu = DSU::new(self.data.len());
        let mut edges: Vec<HeapItem> = vec![];
        let mut mst_edges = vec![];

        // Calculate all distances between points and sort them
        for i in 0..self.data.len() {
            for j in i + 1..self.data.len() {
                let dist2 = Point3d::squared_distance(self.data[i], self.data[j]);
                edges.push((dist2, i, j));
            }
        }
        edges.sort();

        // https://cp-algorithms.com/graph/mst_kruskal.html
        // Minimum Spanning Tree edges - creates a tree where all nodes are connected
        for &(dist2, a, b) in &edges {
            if dsu.find(a) != dsu.find(b) {
                dsu.union(a, b);
                mst_edges.push((dist2, a, b));

                if mst_edges.len() == self.data.len() - 1 {
                    break;
                }
            }
        }

        let (_, last_a, last_b) = mst_edges.last().unwrap();
        ((self.data[*last_a].x * self.data[*last_b].x) as usize).into()
    }
}

// For assisting functions
impl Playground {}

// cargo test --lib day_XX
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::utils::read_data_from_file;

    #[test]
    fn test() {
        let test_data = read_data_from_file("input/2025/test/08.txt");
        let queue = Playground::fro(&test_data);

        assert_eq!(queue.silver(), TaskResult::Usize(40));
        assert_eq!(queue.gold(), TaskResult::Usize(25272));
    }

    #[test]
    fn real() {
        let real_data = read_data_from_file("input/2025/real/08.txt");
        let queue = Playground::fro(&real_data);

        assert_eq!(queue.silver(), TaskResult::Usize(54600));
        assert_eq!(queue.gold(), TaskResult::Usize(107256172));
    }
}
