pub trait Drainable {
    type Item;
    fn len(&self) -> usize;
    fn remove(&mut self) -> Option<Self::Item>;
}

impl<T: Drainable> Drainable for &mut T {
    type Item = T::Item;

    fn len(&self) -> usize {
        (**self).len()
    }

    fn remove(&mut self) -> Option<Self::Item> {
        (**self).remove()
    }
}

pub struct DrainableIter<T> {
    pq: T,
}

impl<T> DrainableIter<T> {
    pub fn new(pq: T) -> Self {
        Self { pq }
    }
}

impl<T: Drainable> Iterator for DrainableIter<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.pq.remove()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.pq.len();
        (n, Some(n))
    }
}

impl<T: Drainable> ExactSizeIterator for DrainableIter<T> {}
