use super::node::Node;
use std::borrow::Borrow;

// Implements *Reb-Black Binary Search Tree* container.
// Partially based on https://algs4.cs.princeton.edu/33balanced/.
pub struct RBTree<K, V> {
    root: Node<K, V>,
}

impl<K, V> RBTree<K, V> {
    pub fn new() -> Self {
        Self { root: Node::none() }
    }

    pub fn len(&self) -> usize {
        self.root.len()
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    pub(crate) fn root(&self) -> &Node<K, V> {
        &self.root
    }

    pub(crate) fn root_mut(&mut self) -> &mut Node<K, V> {
        &mut self.root
    }

    pub(crate) fn take_root(&mut self) -> Node<K, V> {
        let mut node = Node::none();
        node.replace_content(self.root_mut().take_content());
        node
    }
}

impl<K: Ord, V> RBTree<K, V> {
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
    {
        self.root.get(key)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
    {
        self.root.get_mut(key)
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<(K, V)> {
        self.root.insert(key, val)
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
    {
        self.root.remove(key)
    }

    pub fn clear(&mut self) {
        self.root_mut().take_content();
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for RBTree<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut tree = Self::new();
        for (k, v) in iter {
            tree.insert(k, v);
        }
        tree
    }
}

impl<K: Ord, V, const N: usize> From<[(K, V); N]> for RBTree<K, V> {
    fn from(arr: [(K, V); N]) -> Self {
        let mut tree = Self::new();
        for (k, v) in arr {
            tree.insert(k, v);
        }
        tree
    }
}

impl<K, V> Default for RBTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
