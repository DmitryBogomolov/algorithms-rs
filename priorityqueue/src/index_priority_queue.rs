use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

// Implements *Index Priority Queue* container.
// https://algs4.cs.princeton.edu/24pq/
pub struct IndexPriorityQueue<K, T, F>
where
    K: Hash + Eq + Clone,
    F: FnMut(&T, &T) -> bool,
{
    heap: Vec<(K, T)>,
    idx: HashMap<K, usize>,
    is_ord: F,
}

// TODO: ref in idx.key (no Clone)
// TODO: copypaste?

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

    pub fn size(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
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
        if self.heap.is_empty() {
            return None;
        }
        Some(&self.heap[0])
    }

    pub fn remove(&mut self) -> Option<(K, T)> {
        if self.heap.is_empty() {
            return None;
        }
        self.idx.remove(&self.heap[0].0);
        let element = self.heap.swap_remove(0);
        if !self.heap.is_empty() {
            self.idx.insert(self.heap[0].0.clone(), 0);
            self.sink(0);
        }
        Some(element)
    }

    pub fn clear(&mut self) {
        self.heap.clear();
        self.idx.clear();
    }

    pub fn peek_idx<Q>(&self, idx: &Q) -> Option<&(K, T)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        let k = self.idx.get(idx)?;
        Some(&self.heap[*k])
    }

    pub fn remove_idx<Q>(&mut self, idx: &Q) -> Option<(K, T)>
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        let k = *self.idx.get(idx)?;
        self.idx.remove(self.heap[k].0.borrow());
        let element = self.heap.swap_remove(k);
        if k < self.heap.len() {
            self.idx.insert(self.heap[k].0.clone(), k);
            self.sink(k);
            self.swim(k);
        }
        Some(element)
    }
}

fn swap<K: Hash + Eq, T>(list: &mut Vec<(K, T)>, idx: &mut HashMap<K, usize>, i: usize, j: usize) {
    if i == j {
        return;
    }
    let (key_i, pos_i) = idx.remove_entry(&list[i].0).unwrap();
    let (key_j, pos_j) = idx.remove_entry(&list[j].0).unwrap();
    list.swap(i, j);
    idx.insert(key_j, pos_i);
    idx.insert(key_i, pos_j);
}

impl<K: Hash + Eq + Clone, T: Ord> IndexPriorityQueue<K, T, fn(&T, &T) -> bool> {
    fn lt(a: &T, b: &T) -> bool {
        a < b
    }

    fn gt(a: &T, b: &T) -> bool {
        a > b
    }

    pub fn new_max() -> Self {
        Self::new(Self::lt)
    }

    pub fn new_min() -> Self {
        Self::new(Self::gt)
    }
}

impl<K, T, F> IntoIterator for IndexPriorityQueue<K, T, F>
where
    K: Hash + Eq + Clone,
    F: FnMut(&T, &T) -> bool,
{
    type Item = (K, T);
    type IntoIter = IntoIter<K, T, F>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { pq: self }
    }
}

pub struct IntoIter<K, T, F>
where
    K: Hash + Eq + Clone,
    F: FnMut(&T, &T) -> bool,
{
    pq: IndexPriorityQueue<K, T, F>,
}

impl<K, T, F> Iterator for IntoIter<K, T, F>
where
    K: Hash + Eq + Clone,
    F: FnMut(&T, &T) -> bool,
{
    type Item = (K, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.pq.remove()
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
