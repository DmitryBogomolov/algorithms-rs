use super::common::{Drainable, DrainableIter};

// Implements *Priority Queue* container.
// https://algs4.cs.princeton.edu/24pq/
pub struct PriorityQueue<T, F> {
    heap: Vec<T>,
    is_ord: F,
}

impl<T, F> PriorityQueue<T, F>
where
    F: FnMut(&T, &T) -> bool,
{
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

    fn sink(&mut self, i: usize) {
        let heap = &mut self.heap;
        let len = heap.len();
        let is_ord = &mut self.is_ord;
        let mut parent = i;
        loop {
            let mut child = 2 * parent + 1;
            if child + 1 < len && is_ord(&heap[child], &heap[child + 1]) {
                child += 1;
            }
            if child >= len || !is_ord(&heap[parent], &heap[child]) {
                break;
            }
            heap.swap(parent, child);
            parent = child;
        }
    }

    fn swim(&mut self, i: usize) {
        let heap = &mut self.heap;
        let is_ord = &mut self.is_ord;
        let mut child = i;
        while child > 0 {
            let parent = (child - 1) / 2;
            if !is_ord(&heap[parent], &heap[child]) {
                break;
            }
            heap.swap(parent, child);
            child = parent;
        }
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
        if self.heap.is_empty() {
            return None;
        }
        let element = self.heap.swap_remove(0);
        self.sink(0);
        Some(element)
    }

    pub fn clear(&mut self) {
        self.heap.clear();
    }
}

impl<T: Ord> PriorityQueue<T, fn(&T, &T) -> bool> {
    pub fn new_max() -> Self {
        Self::new(|lhs, rhs| lhs < rhs)
    }

    pub fn new_min() -> Self {
        Self::new(|lhs, rhs| lhs > rhs)
    }
}

impl<T, F> Drainable for PriorityQueue<T, F>
where
    F: FnMut(&T, &T) -> bool,
{
    type Item = T;

    fn len(&self) -> usize {
        PriorityQueue::len(self)
    }

    fn remove(&mut self) -> Option<Self::Item> {
        PriorityQueue::remove(self)
    }
}

impl_into_iter!(
    PriorityQueue<T, F>,
    T,
    [T, F] COND [where F: FnMut(&T, &T) -> bool]
);
