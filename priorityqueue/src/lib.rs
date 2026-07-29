pub struct PriorityQueue<T> {
    items: Vec<T>,
}

impl<T> PriorityQueue<T> {
    pub fn new() -> PriorityQueue<T> {
        todo!("")
    }

    pub fn size(&self) -> usize {
        todo!("")
    }

    pub fn insert(&mut self, element: T) {
        todo!("")
    }

    pub fn peek(&self) -> &T {
        todo!("")
    }

    pub fn remove(&mut self) -> T {
        todo!("")
    }
}

#[cfg(test)]
mod tests {
    use super::PriorityQueue;

    #[test]
    fn empty() {
        let pq = PriorityQueue::<()>::new();
        assert_eq!(pq.size(), 0);
    }
}
