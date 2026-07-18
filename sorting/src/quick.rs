// Sorts using *Quick sort* algorithm.
// https://algs4.cs.princeton.edu/23quicksort/
pub fn sort<T: std::fmt::Debug, F: FnMut(&T, &T) -> bool>(target: &mut [T], mut is_ord: F) {
    // TODO: Insertion cutoff.
    if target.is_empty() {
        return;
    }
    shuffle(target);
    sort_core(target, &mut is_ord, 0, target.len());
}

fn shuffle<T>(target: &mut [T]) {
    let mut rng: u64 = 0x9E37_79B9_7F4A_7C15;
    for i in (1..target.len()).rev() {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        let j = (rng % (i as u64 + 1)) as usize;
        target.swap(i, j);
    }
}

fn sort_core<T: std::fmt::Debug, F: FnMut(&T, &T) -> bool>(
    target: &mut [T],
    is_ord: &mut F,
    lo: usize,
    hi: usize,
) {
    // TODO: Insertion cutoff.
    if lo + 1 >= hi {
        return;
    }
    let (lt, gt) = partition(target, is_ord, lo, hi);
    sort_core(target, is_ord, lo, lt);
    sort_core(target, is_ord, gt + 1, hi);
}

fn partition<T: std::fmt::Debug, F: FnMut(&T, &T) -> bool>(
    target: &mut [T],
    is_ord: &mut F,
    lo: usize,
    hi: usize,
) -> (usize, usize) {
    let mut lt = lo;
    let mut gt = hi - 1;
    let mut i = lo + 1;
    while i <= gt {
        if is_ord(&target[i], &target[lt]) {
            target.swap(lt, i);
            lt += 1;
            i += 1;
        } else if is_ord(&target[lt], &target[i]) {
            target.swap(gt, i);
            gt -= 1;
        } else {
            i += 1;
        }
    }
    (lt, gt)
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
