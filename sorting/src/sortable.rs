pub trait Sortable {
    fn len(&self) -> usize;
    fn is_ord(&self, i: usize, j: usize) -> bool;
    fn swap(&mut self, i: usize, j: usize);
}

#[derive(Debug)]
struct SliceAdapter<'a, T> {
    slice: &'a mut [T],
}

impl<'a, T: PartialOrd + std::fmt::Debug> Sortable for SliceAdapter<'a, T> {
    fn len(&self) -> usize {
        self.slice.len()
    }

    fn is_ord(&self, i: usize, j: usize) -> bool {
        self.slice[i] < self.slice[j]
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.slice.swap(i, j);
        println!("swap {}-{}: {:?}", i, j, self.slice);
    }
}

pub fn from_slice<T: PartialOrd + std::fmt::Debug>(slice: &mut [T]) -> impl Sortable {
    SliceAdapter{ slice }
}
