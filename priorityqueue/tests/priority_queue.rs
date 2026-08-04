use priorityqueue::PriorityQueue;

#[test]
fn empty() {
    let pq = PriorityQueue::<(), _>::new(|_, _| false);
    assert!(pq.is_empty());
    assert_eq!(pq.len(), 0);
    assert_eq!(pq.peek(), None);
}

#[test]
fn insert() {
    let mut pq = PriorityQueue::new(|a, b| a < b);

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

fn seed<T, F, I>(mut pq: PriorityQueue<T, F>, items: I) -> PriorityQueue<T, F>
where
    F: FnMut(&T, &T) -> bool,
    I: IntoIterator<Item = T>,
{
    items.into_iter().for_each(|i| pq.insert(i));
    pq
}

#[test]
fn remove() {
    let mut pq = seed(PriorityQueue::new(|a, b| a > b), [4, 6, 4, 3, 8]);

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
    let mut pq = seed(PriorityQueue::new(|a, b| a > b), [4, 6, 4, 3, 8]);

    pq.clear();

    assert!(pq.is_empty());
    assert_eq!(pq.len(), 0);
    assert_eq!(pq.peek(), None);
}

#[test]
fn into_iter() {
    let pq = seed(PriorityQueue::new(|a, b| a > b), [4, 6, 4, 3, 8]);

    let collected: Vec<i32> = pq.into_iter().collect();
    assert_eq!(collected, vec![3, 4, 4, 6, 8]);
}

#[test]
fn drain_full() {
    let mut pq = seed(PriorityQueue::new(|a, b| a > b), [4, 6, 4, 3, 8]);

    let collected: Vec<i32> = pq.drain().collect();
    assert_eq!(collected, [3, 4, 4, 6, 8]);
    assert!(pq.is_empty());
}

#[test]
fn drain_partial() {
    let mut pq = seed(PriorityQueue::new(|a, b| a > b), [4, 6, 4, 3, 8]);

    let collected: Vec<i32> = pq.drain().take(2).collect();
    assert_eq!(collected, vec![3, 4]);
    assert_eq!(pq.len(), 3);
    assert_eq!(pq.remove(), Some(4));
    assert_eq!(pq.remove(), Some(6));
    assert_eq!(pq.remove(), Some(8));
}

#[test]
fn drain_empty() {
    let mut pq: PriorityQueue<i32, _> = PriorityQueue::new(|a, b| a < b);

    let collected: Vec<i32> = pq.drain().collect();
    assert_eq!(collected, []);
}

#[test]
fn max_queue() {
    let pq = seed(PriorityQueue::new_max(), [4, 6, 4, 3, 8]);

    let collected: Vec<i32> = pq.into();
    assert_eq!(collected, [8, 6, 4, 4, 3]);
}

#[test]
fn min_queue() {
    let pq = seed(PriorityQueue::new_min(), [4, 6, 4, 3, 8]);

    let collected: Vec<i32> = pq.into();
    assert_eq!(collected, [3, 4, 4, 6, 8]);
}

#[test]
fn custom_struct() {
    #[derive(Eq, PartialEq, Debug)]
    struct Tester {
        val: i32,
    }
    let mut pq = PriorityQueue::new(|a: &Tester, b: &Tester| a.val > b.val);
    [4, 6, 4, 3, 8]
        .into_iter()
        .for_each(|i| pq.insert(Tester { val: i }));

    assert_eq!(pq.remove().unwrap(), Tester { val: 3 });
    assert_eq!(pq.remove().unwrap(), Tester { val: 4 });
    assert_eq!(pq.remove().unwrap(), Tester { val: 4 });
    assert_eq!(pq.remove().unwrap(), Tester { val: 6 });
    assert_eq!(pq.remove().unwrap(), Tester { val: 8 });
}
