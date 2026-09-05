use super::heap::{Heap, HeapIter};

// Implements *Priority Queue* container.
// https://algs4.cs.princeton.edu/24pq/
#[derive(Clone)]
pub struct PriorityQueue<T, F> {
    heap: Vec<T>,
    is_ord: F,
}

impl<T, F> PriorityQueue<T, F>
where
    F: FnMut(&T, &T) -> bool,
{
    // No FromIterator, From<array>. Because additional `is_ord` argument is required.
    pub fn new(is_ord: F) -> Self {
        Self {
            heap: Vec::new(),
            is_ord,
        }
    }

    pub fn reserve(&mut self, capacity: usize) {
        self.heap.reserve(capacity);
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn clear(&mut self) {
        self.heap.clear();
    }

    pub fn insert(&mut self, element: T) {
        let last = self.heap.len();
        self.heap.push(element);
        self.swim(last);
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.first()
    }

    pub fn remove(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let element = self.heap.swap_remove(0);
        self.sink(0);
        Some(element)
    }

    pub fn drain(&mut self) -> HeapIter<T, &mut F, fn(&T) -> &T> {
        let heap = std::mem::take(&mut self.heap);
        HeapIter::new(heap, &mut self.is_ord, |t| t)
    }
}

impl<T, F> Heap for PriorityQueue<T, F>
where
    F: FnMut(&T, &T) -> bool,
{
    fn heap_len(&self) -> usize {
        self.len()
    }

    fn heap_is_ord(&mut self, lhs: usize, rhs: usize) -> bool {
        (self.is_ord)(&self.heap[lhs], &self.heap[rhs])
    }

    fn heap_swap(&mut self, lhs: usize, rhs: usize) {
        self.heap.swap(lhs, rhs);
    }
}

impl<T> PriorityQueue<T, fn(&T, &T) -> bool>
where
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
impl<T, F> IntoIterator for PriorityQueue<T, F>
where
    F: FnMut(&T, &T) -> bool,
{
    type Item = T;
    type IntoIter = HeapIter<T, F, fn(&T) -> &T>;

    fn into_iter(self) -> Self::IntoIter {
        HeapIter::new(self.heap, self.is_ord, |t| t)
    }
}

impl<T, F> From<PriorityQueue<T, F>> for Vec<T>
where
    F: FnMut(&T, &T) -> bool,
{
    fn from(pq: PriorityQueue<T, F>) -> Self {
        pq.into_iter().collect()
    }
}

impl<T, F> std::fmt::Debug for PriorityQueue<T, F>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.heap.iter()).finish()
    }
}
