use std::{borrow::Borrow, hash::{DefaultHasher, Hash, Hasher}, usize};
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

    fn hash<Q>(&self, key: &Q) -> usize
    where
        Q: Hash + ?Sized,
    {
        if self.slots.is_empty() {
            return usize::MAX;
        }
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize & (self.slots.len() - 1)
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>,
    {
        let h = self.hash(key);
        let data = self.slots.get(h)?.get(key)?;
        Some(data.1)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>,
    {
        let h = self.hash(key);
        let data = self.slots.get_mut(h)?.get_mut(key)?;
        Some(data.1)
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<(K, V)>
    where
        K: Hash + Eq,
    {
        let h = self.hash(&key);
        let slot = self.slots.get_mut(h)?;
        slot.insert(key, val)
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>,
    {
        let h = self.hash(key);
        let slot = self.slots.get_mut(h)?;
        slot.remove(key)
    }
}
