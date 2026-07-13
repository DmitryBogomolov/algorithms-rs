// Sorts using *Quick sort* algorithm.
// https://algs4.cs.princeton.edu/23quicksort/
pub fn sort<T, F: FnMut(&T, &T) -> bool>(target: &mut [T], mut is_ord: F) {
    shuffle(target, &mut is_ord);
}

fn shuffle<T, F: FnMut(&T, &T) -> bool>(target: &mut [T], mut is_ord: F) {
    // TODO: Implement.
}
