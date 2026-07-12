
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
