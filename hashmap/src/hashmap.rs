use std::{borrow::Borrow, hash::{DefaultHasher, Hash, Hasher}};
use super::batch::Batch;

// Implements *Hash Map* container.
// Partially based on https://algs4.cs.princeton.edu/34hash/.
pub struct HashMap<K, V> {
    len: usize,
    slots: Vec<Batch<K, V>>,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            len: 0,
            slots: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get<Q>(&mut self, key: &Q) -> Option<&V>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>,
    {
        let slot = &self.slots[hash(key)];
        let data = slot.get(key)?;
        Some(&data.1)
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<(K, V)>
    where
        K: PartialEq,
    {
        None
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: PartialEq + ?Sized,
        K: Borrow<Q>,
    {
        None
    }
}

fn hash<T: Hash>(t: T) -> usize {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish() as usize
}
