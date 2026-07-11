
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
        let mut v: Vec<i32> = vec![];
        do_sort(v.as_mut_slice(), test_data::asc);
        assert_eq!(v, []);
    }

    #[test]
    fn sort_asc() {
        let mut v = test_data::array();
        do_sort(v.as_mut_slice(), test_data::asc);
        assert_eq!(v, test_data::array_asc());
    }

    #[test]
    fn sort_dsc() {
        let mut v = test_data::array();
        do_sort(v.as_mut_slice(), test_data::dsc);
        assert_eq!(v, test_data::array_dsc());
    }
}
