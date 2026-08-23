use std::{borrow::Borrow, hash::{DefaultHasher, Hash, Hasher}, usize};
use super::batch::Batch;

// Implements *Hash Map* container.
// Partially based on https://algs4.cs.princeton.edu/34hash/.
pub struct HashTable<K, V> {
    len: usize,
    slots: Vec<Batch<K, V>>,
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
        let data = self.slots.get(h)?.get(key)?;
        Some(&data.1)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>,
    {
        let h = self.hash(key);
        let data = self.slots.get_mut(h)?.get_mut(key)?;
        Some(&mut data.1)
    }

    pub fn get_kv<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        Q: Hash + Ord + ?Sized,
        K: Borrow<Q>,
    {
        let h = self.hash(key);
        let data = self.slots.get(h)?.get(key)?;
        Some((&data.0, &data.1))
    }

    pub fn get_kv_mut<Q>(&mut self, key: &Q) -> Option<(&K, &mut V)>
    where
        Q: Hash + Ord + ?Sized,
        K: Borrow<Q>,
    {
        let h = self.hash(key);
        let data = self.slots.get_mut(h)?.get_mut(key)?;
        let ptr: *mut Box<(K, V)> = data;
        unsafe { Some((&(*ptr).0, &mut (*ptr).1)) }
    }    

    fn resize_slots(&mut self, new_size: usize)
    where
        K: Hash + Eq,
    {
        let mut new_slots: Vec<Batch<K, V>> = Vec::new();
        new_slots.resize_with(new_size, || Batch::new());
        let old_slots = std::mem::replace(&mut self.slots, new_slots);
        for slot in old_slots {
            for item in slot.take() {
                let h = self.hash(&item.0);
                self.slots.get_mut(h).unwrap().insert(item);
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
        let ret = slot.insert(Box::new((key, val)));
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
        let ret = slot.remove(key);
        if ret.is_some() {
            self.len -= 1;
            self.check_size();
        }
        ret.map(|t| *t)
    }
}
