use std::borrow::Borrow;

pub struct Batch<K, V> (Option<Vec<Box<(K, V)>>>);

impl<K, V> Batch<K, V> {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn get<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        Q: Eq + ?Sized,
        K: Borrow<Q>,
    {
        let idx = self.find_index(key)?;
        let data = self.0.as_ref().unwrap().get(idx)?;
        Some((&data.0, &data.1))
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<(&K, &mut V)>
    where
        Q: Eq + ?Sized,
        K: Borrow<Q>,
    {
        let idx = self.find_index(key)?;
        let data = self.0.as_mut().unwrap().get_mut(idx)?;
        let ptr: *mut Box<(K, V)> = data;
        unsafe { Some((&(*ptr).0, &mut (*ptr).1)) }
    }

    pub fn insert(&mut self, key: K, val: V) -> Option<(K, V)>
    where
        K: Eq,
    {
        match self.find_index(&key) {
            None => {
                if self.0.is_none() {
                    self.0 = Some(Vec:: new());
                }
                self.0.as_mut().unwrap().push(Box::new((key, val)));
                None
            },
            Some(idx) => {
                let item = self.0.as_mut().unwrap().get_mut(idx).unwrap();
                let k = std::mem::replace(&mut item.as_mut().0, key);
                let v = std::mem::replace(&mut item.as_mut().1, val);
                Some((k, v))
            },
        }
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: Eq + ?Sized,
        K: Borrow<Q>,
    {
        let idx = self.find_index(key)?;
        Some(*self.0.as_mut().unwrap().swap_remove(idx))
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
}
