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

macro_rules! impl_into_iter {
    ($struct_type:ty, $item_type:ty, [$($t_list:tt)+] COND [$($t_bounds:tt)*]) => {
        impl<$($t_list)*> $struct_type $($t_bounds)* {
            pub fn drain(&mut self) -> DrainableIter<&mut Self> {
                DrainableIter::new(self)
            }
        }

        impl<$($t_list)*> IntoIterator for $struct_type $($t_bounds)* {
            type Item = $item_type;
            type IntoIter = DrainableIter<Self>;
            fn into_iter(self) -> Self::IntoIter {
                DrainableIter::new(self)
            }
        }

        impl<$($t_list)*> From<$struct_type> for Vec<$item_type> $($t_bounds)* {
            fn from(pq: $struct_type) -> Self {
                pq.into_iter().collect()
            }
        }
    };
}
