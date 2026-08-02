// Implements *Priority Queue* container.
// https://algs4.cs.princeton.edu/24pq/
pub struct PriorityQueue<T, F: FnMut(&T, &T) -> bool> {
    items: Vec<T>,
    is_ord: F,
}

impl<T, F: FnMut(&T, &T) -> bool> PriorityQueue<T, F> {
    pub fn new(is_ord: F) -> Self {
        Self {
            items: Vec::new(),
            is_ord,
        }
    }

    pub fn reserve(&mut self, capacity: usize) {
        self.items.reserve(capacity);
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn sink(&mut self, i: usize) {
        let items = &mut self.items;
        let len = items.len();
        let is_ord = &mut self.is_ord;
        let mut parent = i;
        loop {
            let mut child = 2 * parent + 1;
            if child + 1 < len && is_ord(&items[child], &items[child + 1]) {
                child += 1;
            }
            if child >= len || !is_ord(&items[parent], &items[child]) {
                break;
            }
            items.swap(parent, child);
            parent = child;
        }
    }

    fn swim(&mut self, i: usize) {
        let items = &mut self.items;
        let is_ord = &mut self.is_ord;
        let mut child = i;
        while child > 0 {
            let parent = (child - 1) / 2;
            if !is_ord(&items[parent], &items[child]) {
                break;
            }
            items.swap(parent, child);
            child = parent;
        }
    }

    pub fn insert(&mut self, element: T) {
        let last = self.items.len();
        self.items.push(element);
        self.swim(last);
    }

    pub fn peek(&self) -> Option<&T> {
        if self.items.is_empty() {
            return None;
        }
        Some(&self.items[0])
    }

    pub fn remove(&mut self) -> Option<T> {
        if self.items.is_empty() {
            return None;
        }
        let element = self.items.swap_remove(0);
        self.sink(0);
        Some(element)
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl<T: Ord> PriorityQueue<T, fn(&T, &T) -> bool> {
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

impl<T, F: FnMut(&T, &T) -> bool> IntoIterator for PriorityQueue<T, F> {
    type Item = T;
    type IntoIter = IntoIter<T, F>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { pq: self }
    }
}

pub struct IntoIter<T, F: FnMut(&T, &T) -> bool> {
    pq: PriorityQueue<T, F>,
}

impl<T, F: FnMut(&T, &T) -> bool> Iterator for IntoIter<T, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pq.remove()
    }
}

impl<T, F: FnMut(&T, &T) -> bool> From<PriorityQueue<T, F>> for Vec<T> {
    fn from(pq: PriorityQueue<T, F>) -> Self {
        pq.into_iter().collect()
    }
}
