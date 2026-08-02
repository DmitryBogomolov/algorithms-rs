use priorityqueue::PriorityQueue;

#[test]
fn empty() {
    let pq = PriorityQueue::<(), _>::new(|_, _| false);
    assert!(pq.is_empty());
    assert_eq!(pq.size(), 0);
    assert_eq!(pq.peek(), None);
}

#[test]
fn insert() {
    let mut pq = PriorityQueue::new(|a, b| a < b);

    pq.insert(4);
    assert!(!pq.is_empty());
    assert_eq!(pq.size(), 1);
    assert_eq!(pq.peek(), Some(&4));

    pq.insert(7);
    assert_eq!(pq.size(), 2);
    assert_eq!(pq.peek(), Some(&7));

    pq.insert(2);
    assert_eq!(pq.size(), 3);
    assert_eq!(pq.peek(), Some(&7));

    pq.insert(7);
    assert_eq!(pq.size(), 4);
    assert_eq!(pq.peek(), Some(&7));

    pq.insert(9);
    assert_eq!(pq.size(), 5);
    assert_eq!(pq.peek(), Some(&9));
}

#[test]
fn remove() {
    let mut pq = PriorityQueue::new(|a, b| a > b);
    for i in [4, 6, 4, 3, 8] {
        pq.insert(i);
    }

    assert_eq!(pq.size(), 5);
    assert_eq!(pq.peek(), Some(&3));

    assert_eq!(pq.remove(), Some(3));
    assert_eq!(pq.size(), 4);

    assert_eq!(pq.remove(), Some(4));
    assert_eq!(pq.size(), 3);

    assert_eq!(pq.remove(), Some(4));
    assert_eq!(pq.size(), 2);

    assert_eq!(pq.remove(), Some(6));
    assert_eq!(pq.size(), 1);

    assert_eq!(pq.remove(), Some(8));
    assert_eq!(pq.size(), 0);

    assert!(pq.is_empty());
    assert_eq!(pq.remove(), None);
}

#[test]
fn clear() {
    let mut pq = PriorityQueue::new(|a, b| a > b);
    for i in [4, 6, 4, 3, 8] {
        pq.insert(i);
    }

    pq.clear();

    assert!(pq.is_empty());
    assert_eq!(pq.size(), 0);
    assert_eq!(pq.peek(), None);
}

#[test]
fn into_iter() {
    let mut pq = PriorityQueue::new(|a, b| a > b);
    for i in [4, 6, 4, 3, 8] {
        pq.insert(i);
    }

    let collected: Vec<i32> = pq.into_iter().collect();
    assert_eq!(collected, vec![3, 4, 4, 6, 8]);
}

#[test]
fn max_queue() {
    let mut pq = PriorityQueue::new_max();
    for i in [4, 6, 4, 3, 8] {
        pq.insert(i);
    }

    let collected: Vec<i32> = pq.into();
    assert_eq!(collected, vec![8, 6, 4, 4, 3]);
}

#[test]
fn min_queue() {
    let mut pq = PriorityQueue::new_min();
    for i in [4, 6, 4, 3, 8] {
        pq.insert(i);
    }

    let collected: Vec<i32> = pq.into();
    assert_eq!(collected, vec![3, 4, 4, 6, 8]);
}

#[test]
fn custom_struct() {
    #[derive(Eq, PartialEq, Debug)]
    struct Tester {
        val: i32,
    }
    let mut pq = PriorityQueue::new(|a: &Tester, b: &Tester| a.val > b.val);
    for i in [4, 6, 4, 3, 8] {
        pq.insert(Tester { val: i });
    }

    assert_eq!(pq.remove().unwrap(), Tester { val: 3 });
    assert_eq!(pq.remove().unwrap(), Tester { val: 4 });
    assert_eq!(pq.remove().unwrap(), Tester { val: 4 });
    assert_eq!(pq.remove().unwrap(), Tester { val: 6 });
    assert_eq!(pq.remove().unwrap(), Tester { val: 8 });
}
