use crate::sortable::{Sortable};

// Sorts using *Insertion sort* algorithm.
// https://algs4.cs.princeton.edu/21elementary/
pub fn sort(target: &mut impl Sortable) {
    let len = target.len();
    if len > 0 {
        sort_core(target, 0, len - 1);
    }
}

fn sort_core(target: &mut impl Sortable, lo: usize, hi: usize) {
    let mut i = lo + 1;
    while i <= hi {
        let mut j = i;
        while j > 0 && target.is_ord(j, j - 1) {
            target.swap(j, j - 1);
            j -= 1;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::sortable::{wrap};

    #[test]
    fn sort() {
        {
            let mut v: Vec<i32> = vec![];
            super::sort(&mut wrap(v.as_mut_slice(), |_: &i32, _: &i32| { false }));
            assert_eq!(v, []);
        }
        {
            let mut v = vec![2, 3, 1, 4];
            super::sort(&mut wrap(v.as_mut_slice(), |i, j| { i < j }));
            assert_eq!(v, [1, 2, 3, 4]);
        }
        {
            let mut v = vec![2, 3, 1, 4];
            super::sort(&mut wrap(v.as_mut_slice(), |i, j| { i > j }));
            assert_eq!(v, [4, 3, 2, 1]);
        }
    }
}
