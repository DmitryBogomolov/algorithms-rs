pub trait Sortable {
    fn len(&self) -> usize;
    fn is_ord(&self, i: usize, j: usize) -> bool;
    fn swap(&mut self, i: usize, j: usize);
}

struct SliceAdapter<'a, T, F: Fn(&T, &T) -> bool> {
    slice: &'a mut [T],
    is_ord_func: F,
}

impl<'a, T: std::fmt::Debug, F: Fn(&T, &T) -> bool> Sortable for SliceAdapter<'a, T, F> {
    fn len(&self) -> usize {
        self.slice.len()
    }

    fn is_ord(&self, i: usize, j: usize) -> bool {
        (self.is_ord_func)(&self.slice[i], &self.slice[j])
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.slice.swap(i, j);
    }
}

pub fn wrap<T: std::fmt::Debug, F: Fn(&T, &T) -> bool>(slice: &mut [T], is_ord: F) -> impl Sortable {
    SliceAdapter{ slice, is_ord_func: is_ord }
}
