use priorityqueue::PriorityQueue;

#[test]
fn empty() {
    let pq = PriorityQueue::<(), _>::new_ord(|_, _| false);
    assert!(pq.is_empty());
    assert_eq!(pq.len(), 0);
    assert_eq!(pq.peek(), None);
}

#[test]
fn insert() {
    let mut pq = PriorityQueue::new_ord(|a, b| a < b);

    pq.insert(4);
    assert!(!pq.is_empty());
    assert_eq!(pq.len(), 1);
    assert_eq!(pq.peek(), Some(&4));

    pq.insert(7);
    assert_eq!(pq.len(), 2);
    assert_eq!(pq.peek(), Some(&7));

    pq.insert(2);
    assert_eq!(pq.len(), 3);
    assert_eq!(pq.peek(), Some(&7));

    pq.insert(7);
    assert_eq!(pq.len(), 4);
    assert_eq!(pq.peek(), Some(&7));

    pq.insert(9);
    assert_eq!(pq.len(), 5);
    assert_eq!(pq.peek(), Some(&9));
}

fn make<T, F, I>(is_ord: F, items: I) -> PriorityQueue<T, F>
where
    F: FnMut(&T, &T) -> bool,
    I: IntoIterator<Item = T>,
{
    let mut pq = PriorityQueue::new_ord(is_ord);
    items.into_iter().for_each(|i| pq.insert(i));
    pq
}

#[test]
fn remove() {
    let mut pq = make(|a, b| a > b, [4, 6, 4, 3, 8]);

    assert_eq!(pq.len(), 5);
    assert_eq!(pq.peek(), Some(&3));

    assert_eq!(pq.remove(), Some(3));
    assert_eq!(pq.len(), 4);

    assert_eq!(pq.remove(), Some(4));
    assert_eq!(pq.len(), 3);

    assert_eq!(pq.remove(), Some(4));
    assert_eq!(pq.len(), 2);

    assert_eq!(pq.remove(), Some(6));
    assert_eq!(pq.len(), 1);

    assert_eq!(pq.remove(), Some(8));
    assert_eq!(pq.len(), 0);

    assert!(pq.is_empty());
    assert_eq!(pq.remove(), None);
}

#[test]
fn clear() {
    let mut pq = make(|a, b| a > b, [4, 6, 4, 3, 8]);

    pq.clear();

    assert!(pq.is_empty());
    assert_eq!(pq.len(), 0);
    assert_eq!(pq.peek(), None);
}

#[test]
fn into_vec() {
    let pq = make(|a, b| a > b, [4, 6, 4, 3, 8]);

    let vec: Vec<_> = pq.into();
    assert_eq!(vec, [3, 4, 4, 6, 8]);
}

#[test]
fn into_iter() {
    let pq = make(|a, b| a > b, [4, 6, 4, 3, 8]);

    let vec: Vec<_> = pq.into_iter().collect();
    assert_eq!(vec, [3, 4, 4, 6, 8]);
}

#[test]
fn drain_full() {
    let mut pq = make(|a, b| a > b, [4, 6, 4, 3, 8]);

    let vec: Vec<i32> = pq.drain().collect();
    assert_eq!(vec, [3, 4, 4, 6, 8]);
    assert_eq!(pq.peek(), None);
    assert!(pq.is_empty());
}

#[test]
fn drain_partial() {
    let mut pq = make(|a, b| a > b, [4, 6, 4, 3, 8]);

    let vec: Vec<i32> = pq.drain().take(2).collect();
    assert_eq!(vec, vec![3, 4]);
    assert!(pq.is_empty());
}

#[test]
fn drain_empty() {
    let mut pq: PriorityQueue<(), _> = PriorityQueue::new_ord(|a, b| a < b);

    let vec: Vec<_> = pq.drain().collect();
    assert_eq!(vec, []);
}

#[test]
fn queue_of_ordered() {
    let mut pq = PriorityQueue::new();
    [4, 6, 4, 3, 8].into_iter().for_each(|t| pq.insert(t));

    let vec: Vec<_> = pq.into();
    assert_eq!(vec, [8, 6, 4, 4, 3]);
}

#[test]
fn custom_struct() {
    #[derive(Eq, PartialEq, Debug)]
    struct Tester {
        val: i32,
    }
    let mut pq = make(
        |a, b| a.val > b.val,
        [4, 6, 4, 3, 8].map(|i| Tester { val: i }),
    );

    assert_eq!(pq.remove(), Some(Tester { val: 3 }));
    assert_eq!(pq.remove(), Some(Tester { val: 4 }));
    assert_eq!(pq.remove(), Some(Tester { val: 4 }));
    assert_eq!(pq.remove(), Some(Tester { val: 6 }));
    assert_eq!(pq.remove(), Some(Tester { val: 8 }));
}

#[test]
fn test_many() {
    let mut pq = PriorityQueue::new_ord(|a, b| a < b);

    for i in 0..400 {
        pq.insert(i + 1);
    }
    assert_eq!(pq.len(), 400);
    assert_eq!(pq.peek(), Some(&400));

    for i in (300..400).rev() {
        assert_eq!(pq.remove(), Some(i + 1));
    }
    assert_eq!(pq.len(), 300);
    assert_eq!(pq.peek(), Some(&300));

    for i in 400..1200 {
        pq.insert(i + 1);
    }
    assert_eq!(pq.len(), 1100);
    assert_eq!(pq.peek(), Some(&1200));

    for i in (800..1200).rev() {
        assert_eq!(pq.remove(), Some(i + 1));
    }
    assert_eq!(pq.len(), 700);
    assert_eq!(pq.peek(), Some(&800));

    let vec: Vec<_> = pq.into();
    let expected: Vec<_> = (0..300).chain(400..800).rev().map(|t| t + 1).collect();
    assert_eq!(vec, expected);
}

#[test]
fn clone() {
    let pq = {
        let mut pq = make(|a, b| a > b, [4, 6, 4, 3, 8]);
        let ret = pq.clone();
        pq.clear();
        ret
    };

    let vec: Vec<_> = pq.into();
    assert_eq!(vec, [3, 4, 4, 6, 8]);
}

#[test]
fn from_iterator_with_func() {
    let pq = PriorityQueue::from_iter_ord(|a, b| a < b, [4, 6, 4, 2, 3, 9, 8]);

    let vec: Vec<_> = pq.into();
    assert_eq!(vec, [9, 8, 6, 4, 4, 3, 2]);
}

#[test]
fn from_iterator() {
    let pq: PriorityQueue<_, _> = [4, 6, 4, 2, 3, 9, 8].into_iter().collect();

    let vec: Vec<_> = pq.into();
    assert_eq!(vec, [9, 8, 6, 4, 4, 3, 2]);
}

#[test]
fn from_array_with_func() {
    let pq = PriorityQueue::from_arr_ord(|a, b| a < b, [4, 6, 4, 2, 3, 9, 8]);

    let vec: Vec<_> = pq.into();
    assert_eq!(vec, [9, 8, 6, 4, 4, 3, 2]);
}

#[test]
fn from_array() {
    let pq: PriorityQueue<_, _> = [4, 6, 4, 2, 3, 9, 8].into();

    let vec: Vec<_> = pq.into();
    assert_eq!(vec, [9, 8, 6, 4, 4, 3, 2]);
}
