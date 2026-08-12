use super::node::Node;

// Implements *Reb-Black Binary Search Tree* container.
// Partially based on https://algs4.cs.princeton.edu/33balanced/.
pub struct RBTree<K, V> {
    root: Node<K, V>,
}

impl<K: Ord, V> RBTree<K, V> {
    pub fn new() -> Self {
        Self { root: Node::none() }
    }

    pub fn len(&self) -> usize {
        self.root.len()
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let r = self.root.get(key);
        r.map(|t| &t.1)
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        let r = self.root.insert(key, val);
        r.map(|t| t.1)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let r = self.root.remove(key);
        r.map(|t| t.1)
    }
}
