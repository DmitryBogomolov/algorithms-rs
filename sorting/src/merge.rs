// Sorts using *Merge sort* algorithm.
// https://algs4.cs.princeton.edu/22mergesort/
pub fn sort<T, F: FnMut(&T, &T) -> bool>(target: &mut [T], mut is_ord: F) {
    if INSERTION_CUTOFF >= target.len() {
        crate::insertion::sort(target, is_ord);
        return;
    }
    let mut indexes: Vec<Index> = (0..target.len()).map(Index).collect();
    let mut aux: Vec<Index> = indexes.clone();
    let mut is_ord_idx = |lhs: &Index, rhs: &Index| is_ord(&target[lhs.0], &target[rhs.0]);
    sort_core(&mut aux, &mut is_ord_idx, &mut indexes, 0, target.len());
    rearrange(target, &mut indexes);
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
        dst[lo..hi].copy_from_slice(&src[lo..hi]);
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
    for dst_item in &mut dst[lo..hi] {
        let j = if i1 >= mid {
            upd(&mut i2)
        } else if i2 >= hi {
            upd(&mut i1)
        } else if is_ord(&src[i2], &src[i1]) {
            upd(&mut i2)
        } else {
            upd(&mut i1)
        };
        *dst_item = src[j];
    }
}

fn upd(k: &mut usize) -> usize {
    let t = *k;
    *k += 1;
    t
}

fn rearrange<T>(target: &mut [T], indexes: &mut [Index]) {
    let mut back = vec![usize::MAX; indexes.len()];
    for (i, idx) in indexes.iter().enumerate() {
        back[idx.0] = i;
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
