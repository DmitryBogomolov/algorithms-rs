pub struct Batch<T>(Option<Box<Content<T>>>);

struct Content<T> {
    data: T,
    link: Batch<T>,
}

impl<T> Batch<T> {
    pub fn none() -> Self {
        Self(None)
    }

    pub fn data(&self) -> &T {
        &self.0.as_ref().expect("must not be empty").data
    }

    pub fn get<F, K>(&self, key_func: F, key: &K) -> Option<&T>
    where
        F: FnMut(&T) -> &K,
        K: Eq + ?Sized,
    {
        let content = self.find(key_func, key)?.0.as_ref().unwrap();
        Some(&content.data)
    }

    pub fn get_mut<F, K>(&mut self, key_func: F, key: &K) -> Option<&mut T>
    where
        F: FnMut(&T) -> &K,
        K: Eq + ?Sized,
    {
        let content = self.find_mut(key_func, key)?.0.as_mut().unwrap();
        Some(&mut content.data)
    }

    pub fn insert<F, K>(&mut self, data: T, mut key_func: F) -> Option<T>
    where
        F: FnMut(&T) -> &K,
        K: Eq + ?Sized,
    {
        let k = key_func(&data);
        if let Some(item) = self.find_mut(key_func, k) {
            let content = item.0.as_mut().unwrap();
            return  Some(std::mem::replace(&mut content.data, data));
        }
        let content = self.0.take();
        self.0 = Some(Box::new(Content { data, link: Self(content) }));
        None
    }

    pub fn remove<F, K>(&mut self, key_func: F, key: &K) -> Option<T>
    where
        F: FnMut(&T) -> &K,
        K: Eq + ?Sized,
    {
        let item = self.find_mut(key_func, key)?;
        let mut content = item.0.take().unwrap();
        item.0 = content.link.0.take();
        Some(content.data)
    }

    fn find<F, K>(&self, mut key_func: F, key: &K) -> Option<&Self>
    where
        F: FnMut(&T) -> &K,
        K: Eq + ?Sized,
    {
        let mut curr = self;
        while curr.0.is_some() {
            if key_func(&curr.0.as_ref().unwrap().data) == key {
                return Some(curr);
            }
            curr = &curr.0.as_ref().unwrap().link;
        }
        None
    }

    fn find_mut<F, K>(&mut self, mut key_func: F, key: &K) -> Option<&mut Self>
    where
        F: FnMut(&T) -> &K,
        K: Eq + ?Sized,
    {
        let mut curr = self;
        while curr.0.is_some() {
            if key_func(&curr.0.as_ref().unwrap().data) == key {
                return Some(curr);
            }
            curr = &mut curr.0.as_mut().unwrap().link;
        }
        None
    }

    pub fn split(self) -> BatchIter<T> {
        BatchIter(self)
    }

    pub fn link(&mut self, mut other: Self) {
        debug_assert!(other.0.as_ref().is_some() && other.0.as_ref().unwrap().link.0.is_none(), "non single item");
        let content = self.0.take();
        self.0 = other.0.take();
        self.0.as_mut().unwrap().link = Self(content);
    }
}

pub struct BatchIter<T>(Batch<T>);

impl<T> Iterator for BatchIter<T> {
    type Item = Batch<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut content = self.0.0.take()?;
        self.0 = std::mem::replace(&mut content.link, Batch::none());
        Some(Batch(Some(content)))
    }
}

pub struct BatchIterOut<T>(Option<Box<Content<T>>>);

impl<T> Iterator for BatchIterOut<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.take() {
            None => None,
            Some(content) => {
                self.0 = content.link.0;
                Some(content.data)
            },
        }
    }
}

pub struct BatchIterRef<'a, T>(Option<&'a Box<Content<T>>>);

impl<'a, T> Iterator for BatchIterRef<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.take() {
            None => None,
            Some(content) => {
                self.0 = content.link.0.as_ref();
                Some(&content.data)
            },
        }
    }
}

pub struct BatchIterMut<'a, T>(Option<&'a mut Box<Content<T>>>);

impl<'a, T> Iterator for BatchIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.take() {
            None => None,
            Some(content) => {
                self.0 = content.link.0.as_mut();
                Some(&mut content.data)
            },
        }
    }
}

impl<T> IntoIterator for Batch<T> {
    type Item = T;
    type IntoIter = BatchIterOut<T>;

    fn into_iter(self) -> Self::IntoIter {
        BatchIterOut(self.0)
    }
}

impl<'a, T> IntoIterator for &'a Batch<T> {
    type Item = &'a T;
    type IntoIter = BatchIterRef<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        BatchIterRef(self.0.as_ref())
    }
}

impl<'a, T> IntoIterator for &'a mut Batch<T> {
    type Item = &'a mut T;
    type IntoIter = BatchIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        BatchIterMut(self.0.as_mut())
    }
}

impl<T: Clone> Clone for Batch<T> {
    fn clone(&self) -> Self {
        match self.0.as_ref() {
            None => Self(None),
            Some(content) => Self(Some(Box::new(Content { data: content.data.clone(), link: content.link.clone() }))),
        }
    }
}
