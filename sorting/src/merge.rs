// Sorts using *Shell sort* algorithm.
// https://algs4.cs.princeton.edu/22mergesort/
pub fn sort<T: Clone, F: FnMut(&T, &T) -> bool>(target: &mut [T], mut is_ord: F) {
    // TODO: Remove Clone.
    let mut aux: Vec<T> = target.iter().map(|t| { t.clone() }).collect();
    sort_core(aux.as_mut_slice(), &mut is_ord, target, 0, target.len());
}

fn sort_core<T: Clone, F: FnMut(&T, &T) -> bool>(
    src: &mut [T],
    is_ord: &mut F,
    dst: &mut [T],
    lo: usize,
    hi: usize,
) {
    if lo + 1 >= hi {
        return;
    }
    let mid = (lo + hi) / 2;
    sort_core(dst, is_ord, src, lo, mid);
    sort_core(dst, is_ord, src, mid, hi);
    if is_ord(&src[mid - 1], &src[mid]) {
        for i in lo..hi {
            dst[i] = src[i].clone();
        }
    } else {
        merge_core(src, is_ord, dst, lo, mid, hi);
    }
}

fn merge_core<T: Clone, F: FnMut(&T, &T) -> bool>(
    src: &mut [T],
    is_ord: &mut F,
    dst: &mut [T],
    lo: usize,
    mid: usize,
    hi: usize,
) {
    let mut i1 = lo;
    let mut i2 = mid;
    for i in lo..hi {
        let j = if i1 >= mid {
            let k = i2;
            i2 += 1;
            k
        } else if i2 >= hi {
            let k = i1;
            i1 += 1;
            k
        } else if is_ord(&src[i1], &src[i2]) {
            let k = i1;
            i1 += 1;
            k
        } else {
            let k = i2;
            i2 += 1;
            k
        };
        dst[i] = src[j].clone();
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
