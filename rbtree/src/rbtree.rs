use super::node::Node;
use std::borrow::Borrow;

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
}
