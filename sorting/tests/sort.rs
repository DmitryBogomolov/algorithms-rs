use sorting;
use rand::{self, RngExt};

type SortFn<T> = fn(&mut [T], fn(&T, &T) -> bool);

fn funcs<T>() -> Vec<SortFn<T>> {
    vec![
        sorting::insertion,
        sorting::shell,
        sorting::merge,
        sorting::quick,
        sorting::heap,
    ]
}

#[test]
fn test_empty_list() {
    for func in funcs() {
        let mut arr: [(); 0] = [];
        func(&mut arr, |_, _| false);
        assert_eq!(arr, []);
    }
}

#[test]
fn test_one_item() {
    for func in funcs() {
        let mut arr = [3.14];
        func(&mut arr, |_, _| false);
        assert_eq!(arr, [3.14]);
    }
}

#[test]
fn test_two_items() {
    for func in funcs() {
        let mut arr = [1.2, 2.3];
        func(&mut arr, |a, b| { a < b });
        assert_eq!(arr, [1.2, 2.3]);
        func(&mut arr, |a, b| { a > b });
        assert_eq!(arr, [2.3, 1.2]);
        func(&mut arr, |a, b| { a < b });
        assert_eq!(arr, [1.2, 2.3]);
    }
}

#[test]
fn test_many_items() {
    let mut rng = rand::rng();
    let sample: Vec<i32> = (0..10000).map(|_| { rng.random_range(0..100) }).collect();
    for func in funcs() {
        let mut arr = sample.clone();
        func(&mut arr, |a, b| { a < b });

        let check = (1..arr.len()).map(|i| { (arr[i - 1] <= arr[i]) as i32 }).sum::<i32>();
        assert_eq!(check as usize, arr.len() - 1);
    }
}
