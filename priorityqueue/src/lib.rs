// Implements *Priority Queue* container.
// https://algs4.cs.princeton.edu/24pq/
pub struct PriorityQueue<T, F: FnMut(&T, &T) -> bool> {
    items: Vec<T>,
    is_ord: F,
}

impl<T, F: FnMut(&T, &T) -> bool> PriorityQueue<T, F> {
    pub fn new(is_ord: F) -> PriorityQueue<T, F> {
        PriorityQueue {
            items: Vec::new(),
            is_ord,
        }
    }

    pub fn with_capacity(is_ord: F, capacity: usize) -> PriorityQueue<T, F> {
        PriorityQueue { 
            items: Vec::with_capacity(capacity),
            is_ord,
        }
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
        self.items.push(element);
        self.swim(self.items.len() - 1);
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

#[cfg(test)]
mod tests {
    use super::PriorityQueue;

    #[test]
    fn empty() {
        let pq = PriorityQueue::<(), _>::new(|_, _| false);
        assert_eq!(pq.is_empty(), true);
        assert_eq!(pq.size(), 0);
        assert_eq!(pq.peek(), None);
    }

    #[test]
    fn add() {
        let mut pq = PriorityQueue::new(|a, b| a < b);
        
        pq.insert(4);
        assert_eq!(pq.is_empty(), false);
        assert_eq!(pq.size(), 1);
        assert_eq!(pq.peek(), Some(&4));

        pq.insert(7);
        assert_eq!(pq.size(), 2);
        assert_eq!(pq.peek(), Some(&7));

        pq.insert(2);
        assert_eq!(pq.size(), 3);
        assert_eq!(pq.peek(), Some(&7));

        pq.insert(7);
        assert_eq!(pq.size(), 4);
        assert_eq!(pq.peek(), Some(&7));

        pq.insert(9);
        assert_eq!(pq.size(), 5);
        assert_eq!(pq.peek(), Some(&9));
    }

    #[test]
    fn remove() {
        let mut pq = PriorityQueue::new(|a, b| a > b);
        pq.insert(4);
        pq.insert(6);
        pq.insert(4);
        pq.insert(3);
        pq.insert(8);

        assert_eq!(pq.size(), 5);
        assert_eq!(pq.peek(), Some(&3));

        assert_eq!(pq.remove(), Some(3));
        assert_eq!(pq.size(), 4);

        assert_eq!(pq.remove(), Some(4));
        assert_eq!(pq.size(), 3);

        assert_eq!(pq.remove(), Some(4));
        assert_eq!(pq.size(), 2);

        assert_eq!(pq.remove(), Some(6));
        assert_eq!(pq.size(), 1);

        assert_eq!(pq.remove(), Some(8));
        assert_eq!(pq.size(), 0);

        assert_eq!(pq.is_empty(), true);
        assert_eq!(pq.remove(), None);
    }

    #[test]
    fn into_iter_drains_in_priority_order() {
        let mut pq = PriorityQueue::new(|a, b| a > b);
        for i in [4, 6, 4, 3, 8] {
            pq.insert(i);
        }

        let collected: Vec<i32> = pq.into_iter().collect();
        assert_eq!(collected, vec![3, 4, 4, 6, 8]);
    }
}
