pub trait Heap {
    fn heap_len(&self) -> usize;
    fn is_heap_ord(&mut self, lhs: usize, rhs: usize) -> bool;
    fn heap_swap(&mut self, lhs: usize, rhs: usize);

    fn sink(&mut self, i: usize) {
        let len = self.heap_len();
        let mut parent = i;
        loop {
            let mut child = 2 * parent + 1;
            if child + 1 < len && self.is_heap_ord(child, child + 1) {
                child += 1;
            }
            if child >= len || !self.is_heap_ord(parent, child) {
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
            if !self.is_heap_ord(parent, child) {
                break;
            }
            self.heap_swap(parent, child);
            child = parent;
        }
    }
}
