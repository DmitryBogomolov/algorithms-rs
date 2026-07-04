use crate::sortable::{Sortable};

// Insertion sorts using *Insertion sort* algorithm.
// https://algs4.cs.princeton.edu/21elementary/
pub fn insertion(target: &mut impl Sortable) {
    insertion_core(target, 0, target.len() - 1);
}

fn insertion_core(target: &mut impl Sortable, lo: usize, hi: usize) {
    let mut i = lo;
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
    use crate::sortable::from_slice;
    use super::*;

    #[test]
    fn sorting() {
        {
            let mut a = vec![2, 3, 1, 4];
            insertion(&mut from_slice(a.as_mut_slice()));
            assert_eq!(a, [1, 2, 3, 4]);    
        }
    }
}
