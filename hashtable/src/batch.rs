pub struct Batch<T>(Option<Box<Content<T>>>);

struct Content<T> {
    data: T,
    link: Batch<T>,
}

impl<T> Batch<T> {
    pub fn none() -> Self {
        Self(None)
    }

    fn is_none(&self) -> bool {
        self.0.is_none()
    }

    fn content(&self) -> &Content<T> {
        self.0.as_ref().unwrap()
    }

    fn content_mut(&mut self) -> &mut Content<T> {
        self.0.as_mut().unwrap()
    }

    fn take_content(&mut self) -> Option<Box<Content<T>>> {
        self.0.take()
    }

    fn set_content(&mut self, content: Option<Box<Content<T>>>) {
        self.0 = content;
    }

    pub fn data(&self) -> &T {
        &self.content().data
    }

    pub fn get<F, K>(&self, key_func: F, key: &K) -> Option<&T>
    where
        F: FnMut(&T) -> &K,
        K: Eq + ?Sized,
    {
        let content = self.find(key_func, key)?.content();
        Some(&content.data)
    }

    pub fn get_mut<F, K>(&mut self, key_func: F, key: &K) -> Option<&mut T>
    where
        F: FnMut(&T) -> &K,
        K: Eq + ?Sized,
    {
        let content = self.find_mut(key_func, key)?.content_mut();
        Some(&mut content.data)
    }

    pub fn insert<F, K>(&mut self, data: T, mut key_func: F) -> Option<T>
    where
        F: FnMut(&T) -> &K,
        K: Eq + ?Sized,
    {
        let k = key_func(&data);
        if let Some(item) = self.find_mut(key_func, k) {
            let content = item.content_mut();
            return Some(std::mem::replace(&mut content.data, data));
        }
        let content = self.take_content();
        self.set_content(make_content(data, Self(content)));
        None
    }

    pub fn remove<F, K>(&mut self, key_func: F, key: &K) -> Option<T>
    where
        F: FnMut(&T) -> &K,
        K: Eq + ?Sized,
    {
        let item = self.find_mut(key_func, key)?;
        let mut content = item.take_content().unwrap();
        item.set_content(content.link.take_content());
        Some(content.data)
    }

    fn find<F, K>(&self, mut key_func: F, key: &K) -> Option<&Self>
    where
        F: FnMut(&T) -> &K,
        K: Eq + ?Sized,
    {
        let mut curr = self;
        while !curr.is_none() {
            if key_func(curr.data()) == key {
                return Some(curr);
            }
            curr = &curr.content().link;
        }
        None
    }

    fn find_mut<F, K>(&mut self, mut key_func: F, key: &K) -> Option<&mut Self>
    where
        F: FnMut(&T) -> &K,
        K: Eq + ?Sized,
    {
        let mut curr = self;
        while !curr.is_none() {
            if key_func(curr.data()) == key {
                return Some(curr);
            }
            curr = &mut curr.content_mut().link;
        }
        None
    }

    pub fn split(self) -> BatchIter<T> {
        BatchIter(self)
    }

    pub fn link(&mut self, mut other: Self) {
        debug_assert!(
            !other.is_none() && other.content().link.is_none(),
            "must be single item"
        );
        let content = self.take_content();
        self.set_content(other.take_content());
        self.content_mut().link = Self(content);
    }
}

fn make_content<T>(data: T, link: Batch<T>) -> Option<Box<Content<T>>> {
    Some(Box::new(Content { data, link }))
}

pub struct BatchIter<T>(Batch<T>);

impl<T> Iterator for BatchIter<T> {
    type Item = Batch<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut content = self.0.take_content()?;
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
            }
        }
    }
}

#[allow(clippy::borrowed_box)]
pub struct BatchIterRef<'a, T>(Option<&'a Box<Content<T>>>);

impl<'a, T> Iterator for BatchIterRef<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.take() {
            None => None,
            Some(content) => {
                self.0 = content.link.0.as_ref();
                Some(&content.data)
            }
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
            }
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
            Some(content) => Self(make_content(content.data.clone(), content.link.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let b: Batch<()> = Batch::none();
        assert_eq!(b.get(|_| &(), &()), None);
    }

    fn key_f<K, V>(t: &(K, V)) -> &K {
        &t.0
    }

    #[test]
    fn insert_and_replace() {
        let mut batch = Batch::none();

        assert_eq!(batch.insert((1, 'a'), key_f), None);
        assert_eq!(batch.data(), &(1, 'a'));
        assert_eq!(batch.get(key_f, &1), Some(&(1, 'a')));

        assert_eq!(batch.insert((1, 'b'), key_f), Some((1, 'a')));
        assert_eq!(batch.data(), &(1, 'b'));
        assert_eq!(batch.get(key_f, &1), Some(&(1, 'b')));

        assert_eq!(batch.insert((1, 'c'), key_f), Some((1, 'b')));
        assert_eq!(batch.data(), &(1, 'c'));
        assert_eq!(batch.get(key_f, &1), Some(&(1, 'c')));
    }

    #[test]
    fn insert_chain() {
        let mut batch = Batch::none();

        assert_eq!(batch.insert((1, 'a'), key_f), None);
        assert_eq!(batch.data(), &(1, 'a'));
        assert_eq!(batch.get(key_f, &1), Some(&(1, 'a')));

        assert_eq!(batch.insert((2, 'b'), key_f), None);
        assert_eq!(batch.data(), &(2, 'b'));
        assert_eq!(batch.get(key_f, &1), Some(&(1, 'a')));
        assert_eq!(batch.get(key_f, &2), Some(&(2, 'b')));

        assert_eq!(batch.insert((3, 'c'), key_f), None);
        assert_eq!(batch.data(), &(3, 'c'));
        assert_eq!(batch.get(key_f, &1), Some(&(1, 'a')));
        assert_eq!(batch.get(key_f, &2), Some(&(2, 'b')));
        assert_eq!(batch.get(key_f, &3), Some(&(3, 'c')));
    }

    #[test]
    fn remove_replaced() {
        let mut batch = Batch::none();

        assert_eq!(batch.remove(key_f, &1), None);

        batch.insert((1, 'a'), key_f);
        assert_eq!(batch.remove(key_f, &1), Some((1, 'a')));
        assert_eq!(batch.remove(key_f, &1), None);

        batch.insert((1, 'a'), key_f);
        batch.insert((1, 'b'), key_f);
        assert_eq!(batch.remove(key_f, &1), Some((1, 'b')));
        assert_eq!(batch.remove(key_f, &1), None);
    }

    #[test]
    fn remove_chain() {
        let mut batch = Batch::none();
        batch.insert((1, 'a'), key_f);
        batch.insert((2, 'b'), key_f);
        batch.insert((3, 'c'), key_f);

        assert_eq!(batch.remove(key_f, &1), Some((1, 'a')));
        assert_eq!(batch.data(), &(3, 'c'));

        assert_eq!(batch.remove(key_f, &3), Some((3, 'c')));
        assert_eq!(batch.data(), &(2, 'b'));

        assert_eq!(batch.remove(key_f, &2), Some((2, 'b')));

        assert_eq!(batch.remove(key_f, &1), None);
        assert_eq!(batch.remove(key_f, &2), None);
        assert_eq!(batch.remove(key_f, &3), None);
    }

    #[test]
    fn get_and_mut() {
        let mut batch = Batch::none();
        batch.insert((1, 'a'), key_f);
        batch.insert((2, 'b'), key_f);
        batch.insert((3, 'c'), key_f);

        assert_eq!(batch.get(key_f, &0), None);
        assert_eq!(batch.get(key_f, &1), Some(&(1, 'a')));
        assert_eq!(batch.get(key_f, &2), Some(&(2, 'b')));
        assert_eq!(batch.get(key_f, &3), Some(&(3, 'c')));

        *batch.get_mut(key_f, &1).unwrap() = (10, 'A');
        *batch.get_mut(key_f, &2).unwrap() = (20, 'B');
        *batch.get_mut(key_f, &3).unwrap() = (30, 'C');

        assert_eq!(batch.get(key_f, &10), Some(&(10, 'A')));
        assert_eq!(batch.get(key_f, &20), Some(&(20, 'B')));
        assert_eq!(batch.get(key_f, &30), Some(&(30, 'C')));

        assert_eq!(batch.data(), &(30, 'C'));
    }

    #[test]
    fn split() {
        let mut batch = Batch::none();
        batch.insert((1, 'a'), key_f);
        batch.insert((2, 'b'), key_f);
        batch.insert((3, 'c'), key_f);

        let items: Vec<_> = batch.split().collect();
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].data(), &(3, 'c'));
        assert_eq!(items[1].data(), &(2, 'b'));
        assert_eq!(items[2].data(), &(1, 'a'));
    }

    #[test]
    fn link() {
        let mut batch = Batch::none();

        batch.link({
            let mut batch = Batch::none();
            batch.insert((1, 'a'), key_f);
            batch
        });
        assert_eq!(batch.data(), &(1, 'a'));

        batch.link({
            let mut batch = Batch::none();
            batch.insert((2, 'b'), key_f);
            batch
        });
        assert_eq!(batch.data(), &(2, 'b'));
        assert_eq!(batch.get(key_f, &1), Some(&(1, 'a')));

        batch.link({
            let mut batch = Batch::none();
            batch.insert((3, 'c'), key_f);
            batch
        });
        assert_eq!(batch.data(), &(3, 'c'));
        assert_eq!(batch.get(key_f, &1), Some(&(1, 'a')));
        assert_eq!(batch.get(key_f, &2), Some(&(2, 'b')));
    }

    #[test]
    fn iter_out() {
        let mut batch = Batch::none();
        batch.insert((1, 'a'), key_f);
        batch.insert((2, 'b'), key_f);
        batch.insert((3, 'c'), key_f);

        let mut items: Vec<_> = batch.into_iter().collect();
        items.sort_by_key(|t| t.0);
        assert_eq!(items, [(1, 'a'), (2, 'b'), (3, 'c')]);
    }

    #[test]
    fn iter_ref() {
        let mut batch = Batch::none();
        batch.insert((1, 'a'), key_f);
        batch.insert((2, 'b'), key_f);
        batch.insert((3, 'c'), key_f);

        let mut items: Vec<_> = (&batch).into_iter().collect();
        items.sort_by_key(|t| t.0);
        assert_eq!(items, [&(1, 'a'), &(2, 'b'), &(3, 'c')]);
    }

    #[test]
    fn iter_mut() {
        let mut batch = Batch::none();
        batch.insert((1, 'a'), key_f);
        batch.insert((2, 'b'), key_f);
        batch.insert((3, 'c'), key_f);

        let mut items: Vec<_> = (&mut batch).into_iter().collect();
        items.sort_by_key(|t| t.0);
        assert_eq!(items, [&mut (1, 'a'), &mut (2, 'b'), &mut (3, 'c')]);
    }
}
