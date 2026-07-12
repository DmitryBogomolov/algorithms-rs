// Sorts using *Merge sort* algorithm.
// https://algs4.cs.princeton.edu/22mergesort/
pub fn sort<T, F: FnMut(&T, &T) -> bool>(target: &mut [T], mut is_ord: F) {
    if target.len() < 2 {
        return;
    }
    let mut indexes: Vec<Index> = (0..target.len()).map(|i| { Index(i) }).collect();
    let mut aux: Vec<Index> = indexes.clone();
    let mut is_ord_idx = |lhs: &Index, rhs: &Index| {
        is_ord(&target[lhs.0], &target[rhs.0])
    };
    sort_core(aux.as_mut_slice(), &mut is_ord_idx, indexes.as_mut_slice(), 0, target.len());
    rearrange(target, indexes.as_mut_slice());
}

#[derive(Copy, Clone)]
struct Index(usize);

const INSERTION_CUTOFF: usize = 7;

fn sort_core(
    src: &mut [Index],
    is_ord: &mut impl FnMut(&Index, &Index) -> bool,
    dst: &mut [Index],
    lo: usize,
    hi: usize,
) {
    if lo + INSERTION_CUTOFF >= hi {
        crate::insertion::sort(&mut dst[lo..hi], is_ord);
        return;
    }
    let mid = (lo + hi) / 2;
    sort_core(dst, is_ord, src, lo, mid);
    sort_core(dst, is_ord, src, mid, hi);
    if is_ord(&src[mid - 1], &src[mid]) {
        for i in lo..hi {
            dst[i] = src[i];
        }
    } else {
        merge_core(src, is_ord, dst, lo, mid, hi);
    }
}

fn merge_core(
    src: &mut [Index],
    is_ord: &mut impl FnMut(&Index, &Index) -> bool,
    dst: &mut [Index],
    lo: usize,
    mid: usize,
    hi: usize,
) {
    let mut i1 = lo;
    let mut i2 = mid;
    for i in lo..hi {
        let j = if i1 >= mid {
            upd(&mut i2)
        } else if i2 >= hi {
            upd(&mut i1)
        } else if is_ord(&src[i2], &src[i1]) {
            upd(&mut i2)
        } else {
            upd(&mut i1)
        };
        dst[i] = src[j];
    }
}

fn upd(k: &mut usize) -> usize {
    let t = *k;
    *k += 1;
    t
}

fn rearrange<T>(target: &mut [T], indexes: &mut [Index]) {
    let mut back = vec![usize::MAX; indexes.len()];
    for i in 0..indexes.len() {
        let k = indexes[i].0;
        back[k] = i;
    }
    for i in 0..indexes.len() - 1 {
        let src = indexes[i].0;
        let dst = i;
        if src != dst {
            target.swap(src, dst);
            indexes[back[dst]].0 = src;
            back[src] = back[dst];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort as do_sort;
    use super::super::test_data;

    #[test]
    fn sort_empty() {
        let mut v = test_data::empty();
        do_sort(v.as_mut_slice(), test_data::asc);
        assert!(test_data::is_sorted(v.as_slice(), test_data::asc));
    }

    #[test]
    fn sort_asc() {
        let mut v = test_data::array();
        do_sort(v.as_mut_slice(), test_data::asc);
        assert!(test_data::is_sorted(v.as_slice(), test_data::asc));
    }

    #[test]
    fn sort_dsc() {
        let mut v = test_data::array();
        do_sort(v.as_mut_slice(), test_data::dsc);
        assert!(test_data::is_sorted(v.as_slice(), test_data::dsc));
    }
}
