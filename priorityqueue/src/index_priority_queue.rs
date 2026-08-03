// Implements *Index Priority Queue* container.
// https://algs4.cs.princeton.edu/24pq/
pub struct IndexPriorityQueue<T, F: FnMut(&T, &T) -> bool> {
    heap: Vec<(usize, T)>,
    is_ord: F,
}

impl<T, F: FnMut(&T, &T) -> bool> IndexPriorityQueue<T, F> {
    pub fn new(is_ord: F) -> Self {
        Self {
            heap: Vec::new(),
            is_ord,
        }
    }

    pub fn reserve(&mut self, capacity: usize) {
        self.heap.reserve(capacity);
    }

    pub fn size(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn sink(&mut self, i: usize) {
        let heap = &mut self.heap;
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
            if !is_ord(&heap[parent].1, &heap[child].1) {
                break;
            }
            heap.swap(parent, child);
            child = parent;
        }
    }

    pub fn insert(&mut self, element: (usize, T)) {
        let last = self.heap.len();
        self.heap.push(element);
        self.swim(last);
    }

    pub fn peek(&self) -> Option<&(usize, T)> {
        if self.heap.is_empty() {
            return None;
        }
        Some(&self.heap[0])
    }

    pub fn remove(&mut self) -> Option<(usize, T)> {
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

impl<T: Ord> IndexPriorityQueue<T, fn(&T, &T) -> bool> {
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

impl<T, F: FnMut(&T, &T) -> bool> IntoIterator for IndexPriorityQueue<T, F> {
    type Item = (usize, T);
    type IntoIter = IntoIter<T, F>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { pq: self }
    }
}

pub struct IntoIter<T, F: FnMut(&T, &T) -> bool> {
    pq: IndexPriorityQueue<T, F>,
}

impl<T, F: FnMut(&T, &T) -> bool> Iterator for IntoIter<T, F> {
    type Item = (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.pq.remove()
    }
}

impl<T, F: FnMut(&T, &T) -> bool> From<IndexPriorityQueue<T, F>> for Vec<(usize, T)> {
    fn from(pq: IndexPriorityQueue<T, F>) -> Self {
        pq.into_iter().collect()
    }
}
