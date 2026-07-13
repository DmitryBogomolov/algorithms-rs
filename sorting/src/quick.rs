// Sorts using *Quick sort* algorithm.
// https://algs4.cs.princeton.edu/23quicksort/
pub fn sort<T: std::fmt::Debug, F: FnMut(&T, &T) -> bool>(target: &mut [T], mut is_ord: F) {
    // TODO: Insertion cutoff.
    if target.is_empty() {
        return;
    }
    println!("0: {:?}", target);
    shuffle(target);
    println!("1: {:?}", target);

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
