use super::heap::{Heap, HeapIter};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

// Implements *Index Priority Queue* container.
// https://algs4.cs.princeton.edu/24pq/
#[derive(Clone)]
pub struct IndexPriorityQueue<K, T, F> {
    heap: Vec<(K, T)>,
    idx: HashMap<K, usize>,
    is_ord: F,
}

impl<K, T, F> IndexPriorityQueue<K, T, F>
where
    K: Hash + Eq + Clone,
    F: FnMut(&T, &T) -> bool,
{
    // No FromIterator, From<array>. Because additional `is_ord` argument is required.
    pub fn new(is_ord: F) -> Self {
        Self {
            heap: Vec::new(),
            idx: HashMap::new(),
            is_ord,
        }
    }

    pub fn reserve(&mut self, capacity: usize) {
        self.heap.reserve(capacity);
        self.idx.reserve(capacity);
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn clear(&mut self) {
        self.take();
    }

    fn take(&mut self) -> (Vec<(K, T)>, HashMap<K, usize>) {
        let heap = std::mem::take(&mut self.heap);
        let idx = std::mem::take(&mut self.idx);
        (heap, idx)
    }

    pub fn insert(&mut self, element: (K, T)) -> Option<(K, T)> {
        let (i, prev) = if let Some(&k) = self.idx.get(&element.0) {
            let prev = std::mem::replace(&mut self.heap[k], element);
            (k, Some(prev))
        } else {
            let k = self.heap.len();
            self.idx.insert(element.0.clone(), k);
            self.heap.push(element);
            (k, None)
        };
        self.fix_heap_order(i);
        prev
    }

    fn fix_heap_order(&mut self, i: usize) {
        self.swim(i);
        self.sink(i);
    }

    pub fn peek(&self) -> Option<(&K, &T)> {
        self.heap.first().map(|t| (&t.0, &t.1))
    }

    pub fn peek_idx<Q>(&self, idx: &Q) -> Option<(&K, &T)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.idx
            .get(idx)
            .map(|k| &self.heap[*k])
            .map(|t| (&t.0, &t.1))
    }

    pub fn remove(&mut self) -> Option<(K, T)> {
        if self.is_empty() {
            return None;
        }
        self.remove_at(0)
    }

    pub fn remove_idx<Q>(&mut self, idx: &Q) -> Option<(K, T)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        let k = *self.idx.get(idx)?;
        self.remove_at(k)
    }

    fn remove_at(&mut self, i: usize) -> Option<(K, T)> {
        self.idx.remove(&self.heap[i].0).expect("bad index access");
        let element = self.heap.swap_remove(i);
        if i < self.heap.len() {
            *self.idx.get_mut(&self.heap[i].0).expect("bad index access") = i;
            self.fix_heap_order(i);
        }
        Some(element)
    }

    pub fn drain(&mut self) -> DrainIter<'_, K, T, F> {
        let (heap, _) = self.take();
        HeapIter::new(heap, &mut self.is_ord, |t| &t.1)
    }
}

pub type DrainIter<'a, K, T, F> = HeapIter<(K, T), &'a mut F, fn(&(K, T)) -> &T>;

impl<K, T, F> Heap for IndexPriorityQueue<K, T, F>
where
    K: Hash + Eq + Clone,
    F: FnMut(&T, &T) -> bool,
{
    fn heap_len(&self) -> usize {
        self.len()
    }

    fn heap_is_ord(&mut self, lhs: usize, rhs: usize) -> bool {
        (self.is_ord)(&self.heap[lhs].1, &self.heap[rhs].1)
    }

    fn heap_swap(&mut self, lhs: usize, rhs: usize) {
        self.heap.swap(lhs, rhs);
        *self
            .idx
            .get_mut(&self.heap[lhs].0)
            .expect("bad index access") = lhs;
        *self
            .idx
            .get_mut(&self.heap[rhs].0)
            .expect("bad index access") = rhs;
    }
}

impl<K, T> IndexPriorityQueue<K, T, fn(&T, &T) -> bool>
where
    K: Hash + Eq + Clone,
    T: Ord,
{
    pub fn new_max() -> Self {
        Self::new(|lhs, rhs| lhs < rhs)
    }

    pub fn new_min() -> Self {
        Self::new(|lhs, rhs| lhs > rhs)
    }
}

// No IntoIterator for &Self, &mut Self and no `iter`, `iter_mut` methods. Because iteration modifies container.
impl<K, T, F> IntoIterator for IndexPriorityQueue<K, T, F>
where
    K: Hash + Eq + Clone,
    F: FnMut(&T, &T) -> bool,
{
    type Item = (K, T);
    type IntoIter = HeapIter<(K, T), F, fn(&(K, T)) -> &T>;

    fn into_iter(self) -> Self::IntoIter {
        HeapIter::new(self.heap, self.is_ord, |t| &t.1)
    }
}

impl<K, T, F> From<IndexPriorityQueue<K, T, F>> for Vec<(K, T)>
where
    K: Hash + Eq + Clone,
    F: FnMut(&T, &T) -> bool,
{
    fn from(pq: IndexPriorityQueue<K, T, F>) -> Self {
        pq.into_iter().collect()
    }
}

impl<Q, K, T, F> std::ops::Index<&Q> for IndexPriorityQueue<K, T, F>
where
    Q: ?Sized + Hash + Eq,
    K: Borrow<Q> + Hash + Eq + Clone,
    F: FnMut(&T, &T) -> bool,
{
    type Output = T;

    fn index(&self, index: &Q) -> &Self::Output {
        self.peek_idx(index).map(|t| t.1).expect("bad index")
    }
}

impl<K, T, F> std::fmt::Debug for IndexPriorityQueue<K, T, F>
where
    K: std::fmt::Debug,
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.heap.iter()).finish()
    }
}
