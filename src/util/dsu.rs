/*
https://www.baeldung.com/cs/disjoint-set-union-data-structure

DSU - also called the Union-Find data structure. Initially, we have a universe U of n
elements: U = {a_1, a_2, ..., a_n}, and we’re interested in separating elements into
independent (non-intersecting) sets. Additionally, we must support the union operation
of two sets. Lastly, for any given element, we must be able to find the set it belongs
to.
*/

pub struct DSU {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);

        for i in 0..n {
            parent.push(i)
        }

        let rank = vec![0; n];
        Self { parent, rank }
    }

    pub fn find(&mut self, x: usize) -> usize {
        // path compression
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let mut root_a = self.find(a);
        let mut root_b = self.find(b);

        if root_a == root_b {
            return;
        }

        // union by rank
        if self.rank[root_a] < self.rank[root_b] {
            std::mem::swap(&mut root_a, &mut root_b);
        }

        self.parent[root_b] = root_a;

        if self.rank[root_a] == self.rank[root_b] {
            self.rank[root_a] += 1;
        }
    }
}
