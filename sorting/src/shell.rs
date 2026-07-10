// Sorts using *Shell sort* algorithm.
// https://algs4.cs.princeton.edu/21elementary/
pub fn sort<T, F: Fn(&T, &T) -> bool>(target: &mut [T], is_ord: F) {
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

#[cfg(test)]
mod tests {
    use super::sort as do_sort;
    use super::super::test_data::test_data;

    #[test]
    fn sort() {
        {
            let mut v: Vec<i32> = vec![];
            do_sort(v.as_mut_slice(), test_data::asc);
            assert_eq!(v, []);
        }
        {
            let mut v = test_data::array();
            do_sort(v.as_mut_slice(), test_data::asc);
            assert_eq!(v, test_data::array_asc());
        }
        {
            let mut v = test_data::array();
            do_sort(v.as_mut_slice(), test_data::dsc);
            assert_eq!(v, test_data::array_dsc());
        }
    }
}
