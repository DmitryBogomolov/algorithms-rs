mod test_data;
use sorting::merge as do_sort;

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
