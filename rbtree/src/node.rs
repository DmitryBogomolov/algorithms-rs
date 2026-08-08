pub struct Node<K, V>(Option<Box<Content<K, V>>>);

enum Clr {
    R,
    B,
}

struct Content<K, V> {
    l_node: Node<K, V>,
    r_node: Node<K, V>,
    clr: Clr,
    size: usize,
    data: (K, V),
}

impl<K: Ord, V> Node<K, V> {
    pub fn none() -> Self {
        Self(None)
    }

    pub fn len(&self) -> usize {
        self.0.as_ref().map_or(0, |t| t.size)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn get(&self, k: &K) -> Option<&(K, V)> {
        None
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<(K, V)> {
        None
    }

    pub fn remove(&mut self, k: &K) -> Option<(K, V)> {
        None
    }
}
