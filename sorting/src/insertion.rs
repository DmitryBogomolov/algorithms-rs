// Sorts using *Insertion sort* algorithm.
// https://algs4.cs.princeton.edu/21elementary/
pub fn sort<T, F: Fn(&T, &T) -> bool>(target: &mut [T], is_ord: F) {
    for i in 1..target.len() {
        let mut j = i;
        while j > 0 && is_ord(&target[j], &target[j - 1]) {
            target.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sort as do_sort;

    #[test]
    fn sort() {
        {
            let mut v: Vec<i32> = vec![];
            do_sort(v.as_mut_slice(), |_: _, _: _| { false });
            assert_eq!(v, []);
        }
        {
            let mut v = vec![2, 3, 1, 4];
            do_sort(v.as_mut_slice(), |i, j| { i < j });
            assert_eq!(v, [1, 2, 3, 4]);
        }
        {
            let mut v = vec![2, 3, 1, 4];
            do_sort(v.as_mut_slice(), |i, j| { i > j });
            assert_eq!(v, [4, 3, 2, 1]);
        }
    }
}
