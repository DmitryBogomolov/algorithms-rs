use super::common::{Drainable, DrainableIter};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

// Implements *Index Priority Queue* container.
// https://algs4.cs.princeton.edu/24pq/
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

    fn sink(&mut self, i: usize) {
        let heap = &mut self.heap;
        let idx = &mut self.idx;
        let len = heap.len();
        let is_ord = &mut self.is_ord;
        let mut parent = i;
        loop {
            let mut child = 2 * parent + 1;
            if child + 1 < len && is_ord(&heap[child].1, &heap[child + 1].1) {
                child += 1;
            }
            if child >= len || !is_ord(&heap[parent].1, &heap[child].1) {
                break;
            }
            swap(heap, idx, parent, child);
            parent = child;
        }
    }

    fn swim(&mut self, i: usize) {
        let heap = &mut self.heap;
        let idx = &mut self.idx;
        let is_ord = &mut self.is_ord;
        let mut child = i;
        while child > 0 {
            let parent = (child - 1) / 2;
            if !is_ord(&heap[parent].1, &heap[child].1) {
                break;
            }
            swap(heap, idx, parent, child);
            child = parent;
        }
    }

    pub fn insert(&mut self, element: (K, T)) {
        if let Some(&k) = self.idx.get(&element.0) {
            self.heap[k] = element;
            self.sink(k);
            self.swim(k);
        } else {
            let k = self.heap.len();
            self.heap.push(element);
            self.idx.insert(self.heap[k].0.clone(), k);
            self.swim(k);
        }
    }

    pub fn peek(&self) -> Option<&(K, T)> {
        self.heap.first()
    }

    pub fn peek_idx<Q>(&self, idx: &Q) -> Option<&(K, T)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.idx.get(idx).and_then(|k| self.heap.get(*k))
    }

    pub fn remove(&mut self) -> Option<(K, T)> {
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

    pub fn clear(&mut self) {
        self.heap.clear();
        self.idx.clear();
    }

    fn remove_at(&mut self, i: usize) -> Option<(K, T)> {
        if self.heap.is_empty() {
            return None;
        }
        self.idx.remove(&self.heap[i].0)?;
        let element = self.heap.swap_remove(i);
        if i < self.heap.len() {
            self.idx.insert(self.heap[i].0.clone(), i);
            self.sink(i);
            self.swim(i);
        }
        Some(element)
    }

    pub fn drain(&mut self) -> DrainableIter<&mut Self> {
        DrainableIter::new(self)
    }
}

fn swap<K: Hash + Eq, T>(list: &mut [(K, T)], idx: &mut HashMap<K, usize>, i: usize, j: usize) {
    if i == j {
        return;
    }
    let (key_i, pos_i) = idx.remove_entry(&list[i].0).unwrap();
    let (key_j, pos_j) = idx.remove_entry(&list[j].0).unwrap();
    list.swap(i, j);
    idx.insert(key_i, pos_j);
    idx.insert(key_j, pos_i);
}

impl<K: Hash + Eq + Clone, T: Ord> IndexPriorityQueue<K, T, fn(&T, &T) -> bool> {
    pub fn new_max() -> Self {
        Self::new(|lhs, rhs| lhs < rhs)
    }

    pub fn new_min() -> Self {
        Self::new(|lhs, rhs| lhs > rhs)
    }
}

impl<K, T, F> Drainable for IndexPriorityQueue<K, T, F>
where
    K: Hash + Eq + Clone,
    F: FnMut(&T, &T) -> bool,
{
    type Item = (K, T);

    fn len(&self) -> usize {
        IndexPriorityQueue::len(self)
    }

    fn remove(&mut self) -> Option<Self::Item> {
        IndexPriorityQueue::remove(self)
    }
}

impl_into_iter!(
    IndexPriorityQueue<K, T, F>,
    (K, T),
    [K, T, F] COND [where K: Hash + Eq + Clone, F: FnMut(&T, &T) -> bool]
);
