
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
        let mut arr = test_data::array();
        let target = &mut arr[0..0];
        do_sort(target, test_data::asc);
        test_data::check_sorted(target, test_data::asc);
    }

    #[test]
    fn sort_one() {
        let mut arr = test_data::array();
        let target = &mut arr[0..1];
        do_sort(target, test_data::asc);
        test_data::check_sorted(target, test_data::asc);
    }

    #[test]
    fn sort_asc() {
        let mut v = test_data::array();
        do_sort(&mut v, test_data::asc);
        test_data::check_sorted(&v, test_data::asc);
    }

    #[test]
    fn sort_dsc() {
        let mut v = test_data::array();
        do_sort(&mut v, test_data::dsc);
        test_data::check_sorted(&v, test_data::dsc);
    }
}
