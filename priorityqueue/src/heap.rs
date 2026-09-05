pub trait Heap {
    fn heap_len(&self) -> usize;
    fn heap_is_ord(&mut self, lhs: usize, rhs: usize) -> bool;
    fn heap_swap(&mut self, lhs: usize, rhs: usize);

    fn sink(&mut self, i: usize) {
        let len = self.heap_len();
        let mut parent = i;
        loop {
            let mut child = 2 * parent + 1;
            if child + 1 < len && self.heap_is_ord(child, child + 1) {
                child += 1;
            }
            if child >= len || !self.heap_is_ord(parent, child) {
                break;
            }
            self.heap_swap(parent, child);
            parent = child;
        }
    }

    fn swim(&mut self, i: usize) {
        let mut child = i;
        while child > 0 {
            let parent = (child - 1) / 2;
            if !self.heap_is_ord(parent, child) {
                break;
            }
            self.heap_swap(parent, child);
            child = parent;
        }
    }
}

pub struct HeapIter<T, FOrd, FKey> {
    heap: Vec<T>,
    ord_func: FOrd,
    key_func: FKey,
}

impl<T, FOrd, FKey> HeapIter<T, FOrd, FKey> {
    pub fn new(heap: Vec<T>, ord_func: FOrd, key_func: FKey) -> Self {
        Self {
            heap,
            ord_func,
            key_func,
        }
    }
}

impl<K, T, FOrd, FKey> Iterator for HeapIter<T, FOrd, FKey>
where
    FOrd: FnMut(&K, &K) -> bool,
    FKey: FnMut(&T) -> &K,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.heap.is_empty() {
            return None;
        }
        let ret = self.heap.swap_remove(0);
        self.sink(0);
        Some(ret)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.heap.len();
        (n, Some(n))
    }
}

impl<K, T, FOrd, FKey> ExactSizeIterator for HeapIter<T, FOrd, FKey>
where
    FOrd: FnMut(&K, &K) -> bool,
    FKey: FnMut(&T) -> &K,
{
}

impl<K, T, FOrd, FKey> Heap for HeapIter<T, FOrd, FKey>
where
    FOrd: FnMut(&K, &K) -> bool,
    FKey: FnMut(&T) -> &K,
{
    fn heap_len(&self) -> usize {
        self.heap.len()
    }

    fn heap_is_ord(&mut self, lhs: usize, rhs: usize) -> bool {
        let lhs_t = (self.key_func)(&self.heap[lhs]);
        let rhs_t = (self.key_func)(&self.heap[rhs]);
        (self.ord_func)(lhs_t, rhs_t)
    }

    fn heap_swap(&mut self, lhs: usize, rhs: usize) {
        self.heap.swap(lhs, rhs);
    }
}
