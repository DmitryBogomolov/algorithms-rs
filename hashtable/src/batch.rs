pub struct Batch<T> (Option<Vec<T>>);

impl<T> Batch<T> {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn get<F, K>(&self, key_func: F, key: &K) -> Option<&T>
    where
        F: FnMut(&T) -> &K,
        K: Eq + ?Sized,
    {
        let idx = self.find_index(key_func, key)?;
        self.0.as_ref().unwrap().get(idx)
    }

    pub fn get_mut<F, K>(&mut self, key_func: F, key: &K) -> Option<&mut T>
    where
        F:FnMut(&T) -> &K,
        K: Eq + ?Sized,
    {
        let idx = self.find_index(key_func, key)?;
        self.0.as_mut().unwrap().get_mut(idx)
    }

    pub fn insert<F, K>(&mut self, data: T, mut key_func: F) -> Option<T>
    where
        F: FnMut(&T) -> &K,
        K: Eq + ?Sized,
    {
        let k = key_func(&data);
        match self.find_index(key_func, k) {
            None => {
                if self.0.is_none() {
                    self.0 = Some(Vec::new());
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

    pub fn remove<F, K>(&mut self, key_func: F, key: &K) -> Option<T>
    where
        F: FnMut(&T) -> &K,
        K: Eq + ?Sized,
    {
        let idx = self.find_index(key_func, key)?;
        Some(self.0.as_mut().unwrap().swap_remove(idx))
    }

    fn find_index<F, K>(&self, mut key_func: F, key: &K) -> Option<usize>
    where
        F: FnMut(&T) -> &K,
        K: Eq + ?Sized,
    {
        if self.0.is_none() {
            return None;
        }
        for (i, data) in self.0.as_ref().unwrap().iter().enumerate() {
            if key_func(data) == key {
                return Some(i);
            }
        }
        None
    }

    pub fn take(self) -> impl Iterator<Item = T> {
        match self.0 {
            None => Vec::new().into_iter(),
            Some(arr) => arr.into_iter(),
        }
    }
}
