use std::{borrow::Borrow, hash::{DefaultHasher, Hash, Hasher}};
use super::batch::Batch;

// Implements *Hash Map* container.
// Partially based on https://algs4.cs.princeton.edu/34hash/.
pub struct HashTable<K, V> {
    pub(crate) len: usize,
    pub(crate) slots: Vec<Batch<Box<(K, V)>>>,
}

const BASE_SLOT_COUNT: usize = 4;
const MIN_BATCH_CAPACITY: usize = 2;
const MAX_BATCH_CAPACITY: usize = 8;

impl<K, V> HashTable<K, V> {
    pub fn new() -> Self {
        let mut ret = Self {
            len: 0,
            slots: Vec::new(),
        };
        ret.clear();
        ret
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        self.len = 0;
        self.slots.clear();
        self.slots.resize_with(BASE_SLOT_COUNT, || Batch::new());
    }

    fn hash<Q>(&self, key: &Q) -> usize
    where
        Q: Hash + ?Sized,
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.slots.len()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>,
    {
        let h = self.hash(key);
        let data = self.slots.get(h)?.get(|t| t.0.borrow(), key)?;
        Some(&data.1)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>,
    {
        let h = self.hash(key);
        let data = self.slots.get_mut(h)?.get_mut(|t| t.0.borrow(), key)?;
        Some(&mut data.1)
    }

    pub fn get_kv<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>,
    {
        let h = self.hash(key);
        let data = self.slots.get(h)?.get(|t| t.0.borrow(), key)?;
        Some((&data.0, &data.1))
    }

    pub fn get_kv_mut<Q>(&mut self, key: &Q) -> Option<(&K, &mut V)>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>,
    {
        let h = self.hash(key);
        let data = self.slots.get_mut(h)?.get_mut(|t| t.0.borrow(), key)?;
        let ptr: *mut Box<(K, V)> = data;
        unsafe { Some((&(*ptr).0, &mut (*ptr).1)) }
    }

    fn resize_slots(&mut self, new_size: usize)
    where
        K: Hash + Eq,
    {
        let mut new_slots: Vec<Batch<Box<(K, V)>>> = Vec::new();
        new_slots.resize_with(new_size, || Batch::new());
        let old_slots = std::mem::replace(&mut self.slots, new_slots);
        for slot in old_slots {
            for item in slot.take() {
                let h = self.hash(&item.0);
                self.slots.get_mut(h).unwrap().insert(item, |t| &t.0);
            }
        }
    }

    fn check_size(&mut self)
    where
        K: Hash + Eq,
    {
        let slot_len = self.slots.len();
        if self.len <= slot_len * MIN_BATCH_CAPACITY {
            let new_slot_len = (slot_len / 2).max(BASE_SLOT_COUNT);
            if new_slot_len < slot_len {
                self.resize_slots(new_slot_len);
            }
        } else if self.len >= slot_len * MAX_BATCH_CAPACITY {
            let new_slot_len = slot_len * 2;
            self.resize_slots(new_slot_len);
        }
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<(K, V)>
    where
        K: Hash + Eq,
    {
        let h = self.hash(&key);
        let slot = self.slots.get_mut(h)?;
        let ret = slot.insert(Box::new((key, val)), |t| &t.0);
        if ret.is_none() {
            self.len += 1;
            self.check_size();
        }
        ret.map(|t| *t)
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: Hash + Eq + ?Sized,
        K: Hash + Eq + Borrow<Q>,
    {
        let h = self.hash(key);
        let slot = self.slots.get_mut(h)?;
        let ret = slot.remove(|t| t.0.borrow(), key);
        if ret.is_some() {
            self.len -= 1;
            self.check_size();
        }
        ret.map(|t| *t)
    }

    // TODO: Return actual type + IntoInterator for HashTable
    pub fn drain(&mut self) -> impl Iterator<Item = (K, V)> {
        self.len = 0;
        self.slots.drain(..).flatten().map(|t| *t)
    }

    // TODO: Return actual type + IntoIterator for &HashTable
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.slots.iter().flatten().map(|t| (&t.0, &t.1))
    }

    // TODO: Return actual type + IntoIterator for &mut HashTable
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> {
        self.slots.iter_mut().flatten().map(|t| {
            let ptr: *mut Box<(K, V)> = t;
            unsafe { (&(*ptr).0, &mut (*ptr).1) }
        })
    }
}

impl<K, V> FromIterator<(K, V)> for HashTable<K, V>
where
    K: Hash + Eq,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut tree = Self::new();
        for (k, v) in iter {
            tree.insert(k, v);
        }
        tree
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for HashTable<K, V>
where
    K: Hash + Eq,
{
    fn from(arr: [(K, V); N]) -> Self {
        arr.into_iter().collect()
    }
}

impl<K, V> Default for HashTable<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Q, K, V> std::ops::Index<&Q> for HashTable<K, V>
where
    Q: Hash + Eq + ?Sized,
    K: Borrow<Q>,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index).unwrap_or_else(|| panic!("bad index"))
    }
}

impl<Q, K, V> std::ops::IndexMut<&Q> for HashTable<K, V>
where
    Q: Hash + Eq + ?Sized,
    K: Borrow<Q>,
{
    fn index_mut(&mut self, index: &Q) -> &mut Self::Output {
        self.get_mut(index).unwrap_or_else(|| panic!("bad index"))
    }
}
