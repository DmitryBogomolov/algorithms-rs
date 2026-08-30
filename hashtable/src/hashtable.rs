use super::batch::Batch;
use std::{
    borrow::Borrow,
    hash::{BuildHasher, Hash, RandomState},
};

type Slot<K, V> = Batch<(K, V)>;
type Slots<K, V> = Vec<Slot<K, V>>;

// Implements *Hash Map* container.
// Partially based on https://algs4.cs.princeton.edu/34hash/.
pub struct HashTable<K, V, H = RandomState> {
    len: usize,
    slots: Slots<K, V>,
    hasher_factory: H,
}

const BASE_SLOT_COUNT: usize = 4;
const MIN_BATCH_CAPACITY: usize = 2;
const MAX_BATCH_CAPACITY: usize = 8;

impl<K, V> HashTable<K, V, RandomState> {
    pub fn new() -> Self {
        Self {
            len: 0,
            slots: init_slots(),
            hasher_factory: RandomState::new(),
        }
    }
}

impl<K, V, H> HashTable<K, V, H> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        self.len = 0;
        self.slots = init_slots();
    }

    pub fn drain(&mut self) -> HashTableIterOut<K, V> {
        let len = std::mem::take(&mut self.len);
        let slots = std::mem::replace(&mut self.slots, init_slots());
        iter_out(slots, len)
    }

    pub fn iter(&self) -> HashTableIterRef<'_, K, V> {
        iter_ref(&self.slots, self.len)
    }

    pub fn iter_mut(&mut self) -> HashTableIterMut<'_, K, V> {
        iter_mut(&mut self.slots, self.len)
    }
}

impl<K, V, H: BuildHasher> HashTable<K, V, H> {
    pub fn with_hasher(hasher_factory: H) -> Self {
        Self {
            len: 0,
            slots: init_slots(),
            hasher_factory,
        }
    }

    fn hash<Q>(&self, key: &Q) -> usize
    where
        Q: Hash + ?Sized,
    {
        debug_assert!(!self.slots.is_empty(), "slots are never empty");
        let hash = self.hasher_factory.hash_one(key);
        (hash as usize) % self.slots.len()
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
        Some((&data.0, &mut data.1))
    }

    fn resize_slots(&mut self, new_size: usize)
    where
        K: Hash + Eq,
    {
        let slots = std::mem::replace(&mut self.slots, make_slots(new_size));
        for slot in slots {
            for item in slot.split() {
                let h = self.hash(&item.data().0);
                self.slots[h].link(item);
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
        let ret = slot.insert((key, val), |t| &t.0);
        if ret.is_none() {
            self.len += 1;
            self.check_size();
        }
        ret
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
        ret
    }
}

fn init_slots<K, V>() -> Slots<K, V> {
    make_slots(BASE_SLOT_COUNT)
}

fn make_slots<K, V>(len: usize) -> Slots<K, V> {
    std::iter::repeat_with(Batch::none).take(len).collect()
}

impl<K, V> FromIterator<(K, V)> for HashTable<K, V, RandomState>
where
    K: Hash + Eq,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut table = Self::new();
        for (k, v) in iter {
            table.insert(k, v);
        }
        table
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for HashTable<K, V, RandomState>
where
    K: Hash + Eq,
{
    fn from(arr: [(K, V); N]) -> Self {
        arr.into_iter().collect()
    }
}

impl<K, V> Default for HashTable<K, V, RandomState> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Q, K, V, H> std::ops::Index<&Q> for HashTable<K, V, H>
where
    Q: Hash + Eq + ?Sized,
    K: Borrow<Q>,
    H: BuildHasher,
{
    type Output = V;

    fn index(&self, index: &Q) -> &Self::Output {
        self.get(index).unwrap_or_else(|| panic!("bad index"))
    }
}

impl<Q, K, V, H> std::ops::IndexMut<&Q> for HashTable<K, V, H>
where
    Q: Hash + Eq + ?Sized,
    K: Borrow<Q>,
    H: BuildHasher,
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
    std::iter::Flatten<std::vec::IntoIter<Slot<K, V>>>,
>;
pub type HashTableIterRef<'a, K, V> = HashTableIter<
    std::iter::Map<
        std::iter::Flatten<std::slice::Iter<'a, Slot<K, V>>>,
        fn(&'a (K, V)) -> (&'a K, &'a V),
    >,
>;
pub type HashTableIterMut<'a, K, V> = HashTableIter<
    std::iter::Map<
        std::iter::Flatten<std::slice::IterMut<'a, Slot<K, V>>>,
        fn(&'a mut (K, V)) -> (&'a K, &'a mut V),
    >,
>;

fn iter_out<K, V>(slots: Slots<K, V>, len: usize) -> HashTableIterOut<K, V> {
    HashTableIterOut {
        iter: slots.into_iter().flatten(),
        len,
    }
}

fn iter_ref<K, V>(slots: &Slots<K, V>, len: usize) -> HashTableIterRef<'_, K, V> {
    HashTableIterRef {
        iter: slots.iter().flatten().map(|t| (&t.0, &t.1)),
        len,
    }
}

fn iter_mut<K, V>(slots: &mut Slots<K, V>, len: usize) -> HashTableIterMut<'_, K, V> {
    HashTableIterMut {
        iter: slots.iter_mut().flatten().map(|t| (&t.0, &mut t.1)),
        len,
    }
}

impl<K, V, H> IntoIterator for HashTable<K, V, H> {
    type Item = (K, V);
    type IntoIter = HashTableIterOut<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        iter_out(self.slots, self.len)
    }
}

impl<'a, K, V, H> IntoIterator for &'a HashTable<K, V, H> {
    type Item = (&'a K, &'a V);
    type IntoIter = HashTableIterRef<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        iter_ref(&self.slots, self.len)
    }
}

impl<'a, K, V, H> IntoIterator for &'a mut HashTable<K, V, H> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = HashTableIterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        iter_mut(&mut self.slots, self.len)
    }
}

impl<K: Clone, V: Clone, H: Clone> Clone for HashTable<K, V, H> {
    fn clone(&self) -> Self {
        Self {
            len: self.len,
            slots: self.slots.clone(),
            hasher_factory: self.hasher_factory.clone(),
        }
    }
}

impl<K: std::fmt::Debug, V: std::fmt::Debug, H> std::fmt::Debug for HashTable<K, V, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}
