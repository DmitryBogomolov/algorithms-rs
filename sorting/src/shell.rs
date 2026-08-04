// Sorts using *Shell sort* algorithm.
// https://algs4.cs.princeton.edu/21elementary/
pub fn sort<T, F: FnMut(&T, &T) -> bool>(target: &mut [T], mut is_ord: F) {
    let len = target.len();
    let mut h: usize = 1;
    while h < len / 3 {
        h = 3 * h - 1;
    }
    while h >= 1 {
        for i in h..len {
            let mut j = i;
            while j >= h && is_ord(&target[j], &target[j - h]) {
                target.swap(j, j - 1);
                j -= h;
            }
        }
        h /= 3;
    }
}
