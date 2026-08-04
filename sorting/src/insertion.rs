// Sorts using *Insertion sort* algorithm.
// https://algs4.cs.princeton.edu/21elementary/
pub fn sort<T, F: FnMut(&T, &T) -> bool>(target: &mut [T], mut is_ord: F) {
    for i in 1..target.len() {
        let mut j = i;
        while j > 0 && is_ord(&target[j], &target[j - 1]) {
            target.swap(j, j - 1);
            j -= 1;
        }
    }
}
