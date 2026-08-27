use super::batch::Batch;
use std::{
    borrow::Borrow,
    hash::{DefaultHasher, Hash, Hasher},
};

type Entry<K, V> = Batch<Box<(K, V)>>;
type Slots<K, V> = Vec<Entry<K, V>>;

// Implements *Hash Map* container.
// Partially based on https://algs4.cs.princeton.edu/34hash/.
pub struct HashTable<K, V> {
    pub(crate) len: usize,
    pub(crate) slots: Slots<K, V>,
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
        let mut new_slots: Slots<K, V> = Vec::new();
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

    pub fn drain(&mut self) -> HashTableIterOut<K, V> {
        let len = std::mem::replace(&mut self.len, 0);
        iter_out(self.slots.drain(..).collect(), len)
    }

    pub fn iter(&self) -> HashTableIterRef<'_, K, V> {
        iter_ref(&self.slots, self.len)
    }

    pub fn iter_mut(&mut self) -> HashTableIterMut<'_, K, V> {
        iter_mut(&mut self.slots, self.len)
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

pub struct HashTableIter<I: Iterator> {
    len: usize,
    iter: I,
}

impl<I: Iterator> Iterator for HashTableIter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.iter.next();
        if ret.is_some() {
            self.len -= 1;
        }
        ret
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<I: Iterator> ExactSizeIterator for HashTableIter<I> {}

pub type HashTableIterOut<K, V> = HashTableIter<
    std::iter::Map<std::iter::Flatten<std::vec::IntoIter<Entry<K, V>>>, fn(Box<(K, V)>) -> (K, V)>,
>;
pub type HashTableIterRef<'a, K, V> = HashTableIter<
    std::iter::Map<
        std::iter::Flatten<std::slice::Iter<'a, Entry<K, V>>>,
        fn(&'a Box<(K, V)>) -> (&'a K, &'a V),
    >,
>;
pub type HashTableIterMut<'a, K, V> = HashTableIter<
    std::iter::Map<
        std::iter::Flatten<std::slice::IterMut<'a, Entry<K, V>>>,
        fn(&'a mut Box<(K, V)>) -> (&'a K, &'a mut V),
    >,
>;

fn iter_out<K, V>(slots: Slots<K, V>, len: usize) -> HashTableIterOut<K, V> {
    HashTableIterOut {
        iter: slots.into_iter().flatten().map(|t| *t),
        len,
    }
}

fn iter_ref<K, V>(slots: &Slots<K, V>, len: usize) -> HashTableIterRef<'_, K, V> {
    HashTableIterRef {
        iter: slots.into_iter().flatten().map(|t| (&t.0, &t.1)),
        len,
    }
}

fn iter_mut<K, V>(slots: &mut Slots<K, V>, len: usize) -> HashTableIterMut<'_, K, V> {
    HashTableIterMut {
        iter: slots.into_iter().flatten().map(|t| {
            let ptr: *mut Box<(K, V)> = t;
            unsafe { (&(*ptr).0, &mut (*ptr).1) }
        }),
        len,
    }
}

impl<K, V> IntoIterator for HashTable<K, V> {
    type Item = (K, V);
    type IntoIter = HashTableIterOut<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        iter_out(self.slots, self.len)
    }
}

impl<'a, K, V> IntoIterator for &'a HashTable<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = HashTableIterRef<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        iter_ref(&self.slots, self.len)
    }
}

impl<'a, K, V> IntoIterator for &'a mut HashTable<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = HashTableIterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        iter_mut(&mut self.slots, self.len)
    }
}
