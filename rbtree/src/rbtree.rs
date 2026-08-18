use super::node::Node;
use std::borrow::Borrow;

// Implements *Reb-Black Binary Search Tree* container.
// Partially based on https://algs4.cs.princeton.edu/33balanced/.
pub struct RBTree<K, V> {
    pub(crate) root: Node<K, V>,
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

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
    {
        Some(self.root.find(key)?.val())
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
    {
        Some(self.root.find_mut(key)?.val_mut())
    }

    pub fn get_kv<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
    {
        let node = self.root.find(key)?;
        Some((node.key(), node.val()))
    }

    pub fn get_kv_mut<Q>(&mut self, key: &Q) -> Option<(&K, &mut V)>
    where
        Q: Ord + ?Sized,
        K: Borrow<Q>,
    {
        let ptr: *mut Node<K, V> = self.root.find_mut(key)?;
        unsafe { Some(((*ptr).key(), (*ptr).val_mut())) }
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<(K, V)>
    where
        K: Ord,
    {
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
        self.root.take_content();
    }
}

impl<K, V> FromIterator<(K, V)> for RBTree<K, V>
where
    K: Ord,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut tree = Self::new();
        for (k, v) in iter {
            tree.insert(k, v);
        }
        tree
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for RBTree<K, V>
where
    K: Ord,
{
    fn from(arr: [(K, V); N]) -> Self {
        arr.into_iter().collect()
    }
}

impl<K, V> Default for RBTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
