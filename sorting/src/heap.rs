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
    let mut p = k;
    loop {
        let mut c = 2 * p + 1;
        if c + 1 < len && is_ord(&target[c], &target[c + 1]) {
            c += 1;
        }
        if c >= len {
            break;
        }
        if is_ord(&target[c], &target[p]) {
            break;
        }
        target.swap(p, c);
        p = c;
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
        test_data::check_sorted_unstable(&v, test_data::asc);
    }

    #[test]
    fn sort_dsc() {
        let mut v = test_data::array();
        do_sort(&mut v, test_data::dsc);
        test_data::check_sorted_unstable(&v, test_data::dsc);
    }
}
