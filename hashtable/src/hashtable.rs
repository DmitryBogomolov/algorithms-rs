use std::{
    borrow::Borrow,
    hash::{BuildHasher, Hash, RandomState},
};

type Bucket<K, V> = Vec<Box<(K, V)>>;
type Buckets<K, V> = Vec<Bucket<K, V>>;

// Implements *Hash Map* container.
// Partially based on https://algs4.cs.princeton.edu/34hash/.
#[derive(Clone)]
pub struct HashTable<K, V, H = RandomState> {
    len: usize,
    buckets: Buckets<K, V>,
    hasher_factory: H,
}

const BASE_BUCKET_COUNT: usize = 4;
const MIN_BUCKET_CAPACITY: usize = 2;
const MAX_BUCKET_CAPACITY: usize = 8;

impl<K, V> HashTable<K, V, RandomState> {
    pub fn new() -> Self {
        Self::with_hasher(RandomState::new())
    }
}

impl<K, V, H> HashTable<K, V, H> {
    pub fn with_hasher(hasher_factory: H) -> Self {
        Self {
            len: 0,
            buckets: init_buckets(),
            hasher_factory,
        }
    }
}

impl<K, V, H: Default> Default for HashTable<K, V, H> {
    fn default() -> Self {
        Self::with_hasher(H::default())
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
        self.buckets = init_buckets();
    }

    pub fn drain(&mut self) -> HashTableIterOut<K, V> {
        let len = std::mem::take(&mut self.len);
        let buckets = std::mem::replace(&mut self.buckets, init_buckets());
        iter_out(buckets, len)
    }

    pub fn iter(&self) -> HashTableIterRef<'_, K, V> {
        iter_ref(&self.buckets, self.len)
    }

    pub fn iter_mut(&mut self) -> HashTableIterMut<'_, K, V> {
        iter_mut(&mut self.buckets, self.len)
    }
}

impl<K, V, H: BuildHasher> HashTable<K, V, H> {
    fn bucket_idx<Q>(&self, key: &Q) -> usize
    where
        Q: Hash + ?Sized,
    {
        let hash = self.hasher_factory.hash_one(key);
        (hash as usize) % self.buckets.len()
    }

    fn find<Q>(&self, key: &Q) -> Option<&(K, V)>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>,
    {
        let idx = self.bucket_idx(key);
        self.buckets[idx]
            .iter()
            .find(|t| t.0.borrow() == key)
            .map(|t| t.as_ref())
    }

    fn find_mut<Q>(&mut self, key: &Q) -> Option<&mut (K, V)>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>,
    {
        let idx = self.bucket_idx(key);
        self.buckets[idx]
            .iter_mut()
            .find(|t| t.0.borrow() == key)
            .map(|t| t.as_mut())
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>,
    {
        self.find(key).map(|t| &t.1)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>,
    {
        self.find_mut(key).map(|t| &mut t.1)
    }

    pub fn get_key_val<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>,
    {
        self.find(key).map(|t| (&t.0, &t.1))
    }

    pub fn get_key_val_mut<Q>(&mut self, key: &Q) -> Option<(&K, &mut V)>
    where
        Q: Hash + Eq + ?Sized,
        K: Borrow<Q>,
    {
        self.find_mut(key).map(|t| (&t.0, &mut t.1))
    }

    fn resize_buckets(&mut self, new_size: usize)
    where
        K: Hash + Eq,
    {
        let buckets = std::mem::replace(&mut self.buckets, make_buckets(new_size));
        for item in buckets.into_iter().flatten() {
            let idx = self.bucket_idx(&item.0);
            add_item(&mut self.buckets[idx], item);
        }
    }

    fn adjust_buckets_size(&mut self)
    where
        K: Hash + Eq,
    {
        let buckets_len = self.buckets.len();
        if self.len <= buckets_len * MIN_BUCKET_CAPACITY {
            let new_buckets_len = (buckets_len / 2).max(BASE_BUCKET_COUNT);
            if new_buckets_len < buckets_len {
                self.resize_buckets(new_buckets_len);
            }
        } else if self.len >= buckets_len * MAX_BUCKET_CAPACITY {
            let new_buckets_len = buckets_len * 2;
            self.resize_buckets(new_buckets_len);
        }
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<(K, V)>
    where
        K: Hash + Eq,
    {
        if let Some(item) = self.find_mut(&key) {
            let prev_key = std::mem::replace(&mut item.0, key);
            let prev_val = std::mem::replace(&mut item.1, val);
            Some((prev_key, prev_val))
        } else {
            let idx = self.bucket_idx(&key);
            add_item(&mut self.buckets[idx], Box::new((key, val)));
            self.len += 1;
            self.adjust_buckets_size();
            None
        }
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: Hash + Eq + ?Sized,
        K: Hash + Eq + Borrow<Q>,
    {
        let idx = self.bucket_idx(key);
        if let Some(k) = self.buckets[idx].iter().position(|t| t.0.borrow() == key) {
            let item = rem_item(&mut self.buckets[idx], k);
            self.len -= 1;
            self.adjust_buckets_size();
            Some(*item)
        } else {
            None
        }
    }
}

fn init_buckets<K, V>() -> Buckets<K, V> {
    make_buckets(BASE_BUCKET_COUNT)
}

fn make_buckets<K, V>(len: usize) -> Buckets<K, V> {
    std::iter::repeat_with(Vec::default).take(len).collect()
}

fn add_item<T>(bucket: &mut Vec<T>, item: T) {
    let last = bucket.len();
    bucket.push(item);
    bucket.swap(0, last);
}

fn rem_item<T>(bucket: &mut Vec<T>, idx: usize) -> T {
    bucket.swap_remove(idx)
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

impl<K, V> From<HashTable<K, V>> for Vec<(K, V)> {
    fn from(table: HashTable<K, V>) -> Self {
        table.into_iter().collect()
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
        self.get(index).expect("bad index")
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
    std::iter::Map<std::iter::Flatten<std::vec::IntoIter<Bucket<K, V>>>, fn(Box<(K, V)>) -> (K, V)>,
>;
pub type HashTableIterRef<'a, K, V> = HashTableIter<
    std::iter::Map<
        std::iter::Flatten<std::slice::Iter<'a, Bucket<K, V>>>,
        fn(&'a Box<(K, V)>) -> (&'a K, &'a V),
    >,
>;
pub type HashTableIterMut<'a, K, V> = HashTableIter<
    std::iter::Map<
        std::iter::Flatten<std::slice::IterMut<'a, Bucket<K, V>>>,
        fn(&'a mut Box<(K, V)>) -> (&'a K, &'a mut V),
    >,
>;

fn iter_out<K, V>(buckets: Buckets<K, V>, len: usize) -> HashTableIterOut<K, V> {
    HashTableIterOut {
        iter: buckets.into_iter().flatten().map(|t| *t),
        len,
    }
}

fn iter_ref<K, V>(buckets: &Buckets<K, V>, len: usize) -> HashTableIterRef<'_, K, V> {
    HashTableIterRef {
        iter: buckets.iter().flatten().map(|t| (&t.0, &t.1)),
        len,
    }
}

fn iter_mut<K, V>(buckets: &mut Buckets<K, V>, len: usize) -> HashTableIterMut<'_, K, V> {
    HashTableIterMut {
        iter: buckets.iter_mut().flatten().map(|t| (&t.0, &mut t.1)),
        len,
    }
}

impl<K, V, H> IntoIterator for HashTable<K, V, H> {
    type Item = (K, V);
    type IntoIter = HashTableIterOut<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        iter_out(self.buckets, self.len)
    }
}

impl<'a, K, V, H> IntoIterator for &'a HashTable<K, V, H> {
    type Item = (&'a K, &'a V);
    type IntoIter = HashTableIterRef<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        iter_ref(&self.buckets, self.len)
    }
}

impl<'a, K, V, H> IntoIterator for &'a mut HashTable<K, V, H> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = HashTableIterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        iter_mut(&mut self.buckets, self.len)
    }
}

impl<K: std::fmt::Debug, V: std::fmt::Debug, H> std::fmt::Debug for HashTable<K, V, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}
