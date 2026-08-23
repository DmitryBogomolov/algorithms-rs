use std::borrow::Borrow;

pub struct Batch<K, V> (Option<Vec<Box<(K, V)>>>);

impl<K, V> Batch<K, V> {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&Box<(K, V)>>
    where
        Q: Eq + ?Sized,
        K: Borrow<Q>,
    {
        let idx = self.find_index(key)?;
        self.0.as_ref().unwrap().get(idx)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Box<(K, V)>>
    where
        Q: Eq + ?Sized,
        K: Borrow<Q>,
    {
        let idx = self.find_index(key)?;
        self.0.as_mut().unwrap().get_mut(idx)
    }

    pub fn insert(&mut self, data: Box<(K, V)>) -> Option<Box<(K, V)>>
    where
        K: Eq,
    {
        match self.find_index(&data.0) {
            None => {
                if self.0.is_none() {
                    self.0 = Some(Vec:: new());
                }
                self.0.as_mut().unwrap().push(data);
                None
            },
            Some(idx) => {
                let item = self.0.as_mut().unwrap().get_mut(idx).unwrap();
                Some(std::mem::replace(item, data))
            },
        }
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<Box<(K, V)>>
    where
        Q: Eq + ?Sized,
        K: Borrow<Q>,
    {
        let idx = self.find_index(key)?;
        Some(self.0.as_mut().unwrap().swap_remove(idx))
    }

    fn find_index<Q>(&self, key: &Q) -> Option<usize>
    where
        Q: Eq + ?Sized,
        K: Borrow<Q>,
    {
        if self.0.is_none() {
            return None;
        }
        for (i, data) in self.0.as_ref().unwrap().iter().enumerate() {
            if data.as_ref().0.borrow() == key {
                return Some(i);
            }
        }
        None
    }

    pub fn take(self) -> impl Iterator<Item = Box<(K, V)>> {
        match self.0 {
            None => Vec::new().into_iter(),
            Some(arr) => arr.into_iter(),
        }
    }
}
