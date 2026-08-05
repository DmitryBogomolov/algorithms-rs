// Sorts using *Heap sort* algorithm.
// https://algs4.cs.princeton.edu/24pq/
pub fn sort<T, F: FnMut(&T, &T) -> bool>(target: &mut [T], mut is_ord: F) {
    let len = target.len();
    for i in (0..len / 2).rev() {
        sink(target, &mut is_ord, i, len);
    }
    for i in (1..len).rev() {
        target.swap(0, i);
        sink(target, &mut is_ord, 0, i);
    }
}

// p -> c1 = 2p + 1, c2 = 2p + 2
// c1, c2 -> p = (c - 1) / 2
// heap condition: is_ord(c1, p), is_ord(c2, p)
fn sink<T, F: FnMut(&T, &T) -> bool>(target: &mut [T], is_ord: &mut F, k: usize, len: usize) {
    let mut parent = k;
    loop {
        let mut child = 2 * parent + 1;
        if child + 1 < len && is_ord(&target[child], &target[child + 1]) {
            child += 1;
        }
        if child >= len {
            break;
        }
        if is_ord(&target[child], &target[parent]) {
            break;
        }
        target.swap(parent, child);
        parent = child;
    }
}
