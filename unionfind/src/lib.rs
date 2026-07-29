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
        if i >= self.roots.len() {
            panic!("out of range: {}", i)
        }
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

#[cfg(test)]
mod tests {
    use super::UnionFind;

    #[test]
    fn empty() {
        let uf = UnionFind::new(0);

        assert_eq!(uf.size(), 0);
        assert_eq!(uf.count(), 0);
    }

    #[test]
    fn one_item() {
        let uf = UnionFind::new(1);

        assert_eq!(uf.size(), 1);
        assert_eq!(uf.count(), 1);
        assert_eq!(uf.find(0), 0);
    }

    fn groups(uf: &UnionFind) -> Vec<usize> {
        (0..uf.size()).map(|i| uf.find(i)).collect()
    }

    #[test]
    fn unions() {
        let mut uf = UnionFind::new(5);

        assert_eq!(uf.size(), 5);
        assert_eq!(uf.count(), 5);
        assert_eq!(groups(&uf), vec![0, 1, 2, 3, 4]);

        uf.union(0, 3);
        assert_eq!(uf.size(), 5);
        assert_eq!(uf.count(), 4);
        assert_eq!(groups(&uf), vec![0, 1, 2, 0, 4]);

        uf.union(1, 4);
        assert_eq!(uf.size(), 5);
        assert_eq!(uf.count(), 3);
        assert_eq!(groups(&uf), vec![0, 1, 2, 0, 1]);

        uf.union(2, 4);
        assert_eq!(uf.size(), 5);
        assert_eq!(uf.count(), 2);
        assert_eq!(groups(&uf), vec![0, 1, 1, 0, 1]);

        uf.union(3, 1);
        assert_eq!(uf.size(), 5);
        assert_eq!(uf.count(), 1);
        assert_eq!(groups(&uf), vec![1, 1, 1, 1, 1]);
    }

    #[test]
    fn collapse() {
        let mut uf = UnionFind::new(5);

        uf.union(0, 1);
        uf.union(2, 3);
        uf.union(1, 4);
        uf.union(3, 0);

        uf.collapse();

        assert_eq!(uf.count(), 1);
        assert_eq!(groups(&uf), vec![0, 0, 0, 0, 0]);
    }
}
