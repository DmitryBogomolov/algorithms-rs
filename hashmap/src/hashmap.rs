use std::borrow::Borrow;

// Implements *Hash Map* container.
// Partially based on https://algs4.cs.princeton.edu/34hash/.
pub struct HashMap<K, V> {
    len: usize,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

struct Group<K, V>(Option<Box<Content<K, V>>>);

struct Content<K, V> {
    key: K,
    val: V,
    next: Group<K, V>,
}

impl<K, V> Group<K, V> {
    fn get<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        Q: PartialEq + ?Sized,
        K: Borrow<Q>,
    {
        let mut curr = self;
        while let Some(content) = &curr.0 {
            if content.key.borrow() == key {
                return Some((&content.key, &content.val));
            }
            curr = &content.next;
        }
        None
    }

    fn get_mut<Q>(&mut self, key: &Q) -> Option<(&K, &mut V)>
    where
        Q: PartialEq + ?Sized,
        K: Borrow<Q>,
    {
        let mut curr = self;
        while let Some(content) = &mut curr.0 {
            if content.key.borrow() == key {
                return Some((&content.key, &mut content.val));
            }
            curr = &mut content.next;
        }
        None
    }

    fn insert(&mut self, key: K, val: V) -> Option<(K, V)>
    where
        K: PartialEq,
    {
        None
    }

    fn remove<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: PartialEq + ?Sized,
        K: Borrow<Q>,
    {
        None
    }
}