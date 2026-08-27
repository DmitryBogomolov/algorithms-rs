pub struct Batch<T>(Option<Vec<T>>);

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
        F: FnMut(&T) -> &K,
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
            }
            Some(idx) => {
                let item = self.0.as_mut().unwrap().get_mut(idx).unwrap();
                Some(std::mem::replace(item, data))
            }
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
        let vec = self.0.as_ref()?;
        for (i, data) in vec.iter().enumerate() {
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

pub struct BatchIterOut<T>(Option<std::vec::IntoIter<T>>);

impl<T> Iterator for BatchIterOut<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut()?.next()
    }
}

pub struct BatchIterRef<'a, T>(Option<std::slice::Iter<'a, T>>);

impl<'a, T> Iterator for BatchIterRef<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut()?.next()
    }
}

pub struct BatchIterMut<'a, T>(Option<std::slice::IterMut<'a, T>>);

impl<'a, T> Iterator for BatchIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut()?.next()
    }
}

impl<T> IntoIterator for Batch<T> {
    type Item = T;
    type IntoIter = BatchIterOut<T>;

    fn into_iter(self) -> Self::IntoIter {
        BatchIterOut(self.0.map(|t| t.into_iter()))
    }
}

impl<'a, T> IntoIterator for &'a Batch<T> {
    type Item = &'a T;
    type IntoIter = BatchIterRef<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        BatchIterRef(self.0.as_ref().map(|t| t.iter()))
    }
}

impl<'a, T> IntoIterator for &'a mut Batch<T> {
    type Item = &'a mut T;
    type IntoIter = BatchIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        BatchIterMut(self.0.as_mut().map(|t| t.iter_mut()))
    }
}
