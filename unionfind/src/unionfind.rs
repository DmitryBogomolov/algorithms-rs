// Union-Find implementation.
// https://algs4.cs.princeton.edu/15uf/
pub struct UnionFind {
    size: usize,
    count: usize,
    roots: Vec<usize>,
    sizes: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> UnionFind {
        UnionFind {
            size,
            count: size,
            roots: (0..size).collect(),
            sizes: vec![1; size],
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn count(&self) -> usize {
        self.count
    }

    fn validate(&self, i: usize) {
        assert!(i < self.roots.len(), "out of range: {}", i);
    }

    fn is_root(&self, i: usize) -> bool {
        i == self.roots[i]
    }

    fn get_root(&self, i: usize) -> usize {
        let mut k = i;
        while !self.is_root(k) {
            k = self.roots[k]
        }
        k
    }

    pub fn find(&self, i: usize) -> usize {
        self.validate(i);
        self.get_root(i)
    }

    pub fn union(&mut self, i: usize, j: usize) {
        self.validate(i);
        self.validate(j);
        let i_root = self.get_root(i);
        let j_root = self.get_root(j);

        let (child, parent) = if self.sizes[i_root] < self.sizes[j_root] {
            (i_root, j_root)
        } else {
            (j_root, i_root)
        };
        self.roots[child] = parent;
        self.sizes[parent] += self.sizes[child];
        self.count -= 1;
    }

    pub fn collapse(&mut self) {
        for i in 0..self.roots.len() {
            if !self.is_root(i) && !self.is_root(self.roots[i]) {
                self.roots[i] = self.get_root(i);
            }
        }
    }
}
