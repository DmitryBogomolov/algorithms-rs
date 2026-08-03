use priorityqueue::IndexPriorityQueue;


#[test]
fn empty() {
    let pq = IndexPriorityQueue::<(), _>::new(|_, _| false);
    assert!(pq.is_empty());
    assert_eq!(pq.size(), 0);
    assert_eq!(pq.peek(), None);
}

#[test]
fn insert() {
    let mut pq = IndexPriorityQueue::new(|a, b| a < b);

    pq.insert((100, 4));
    assert!(!pq.is_empty());
    assert_eq!(pq.size(), 1);
    assert_eq!(pq.peek(), Some(&(100, 4)));

    pq.insert((200, 7));
    assert_eq!(pq.size(), 2);
    assert_eq!(pq.peek(), Some(&(200, 7)));

    pq.insert((300, 2));
    assert_eq!(pq.size(), 3);
    assert_eq!(pq.peek(), Some(&(200, 7)));

    pq.insert((400, 7));
    assert_eq!(pq.size(), 4);
    assert_eq!(pq.peek(), Some(&(200, 7)));

    pq.insert((500, 9));
    assert_eq!(pq.size(), 5);
    assert_eq!(pq.peek(), Some(&(500, 9)));
}

fn seed<T, F, I>(mut pq: IndexPriorityQueue<T, F>, items: I) -> IndexPriorityQueue<T, F>
where
    F: FnMut(&T, &T) -> bool,
    I: IntoIterator<Item = (usize, T)>,
{
    items.into_iter().for_each(|i| pq.insert(i));
    pq
}

#[test]
fn remove() {
    let mut pq = seed(IndexPriorityQueue::new(|a, b| a > b), [(100, 4), (200, 6), (300, 4), (400, 3), (500, 8)]);

    assert_eq!(pq.size(), 5);
    assert_eq!(pq.peek(), Some(&(400, 3)));

    assert_eq!(pq.remove(), Some((400, 3)));
    assert_eq!(pq.size(), 4);

    assert_eq!(pq.remove(), Some((100, 4)));
    assert_eq!(pq.size(), 3);

    assert_eq!(pq.remove(), Some((300, 4)));
    assert_eq!(pq.size(), 2);

    assert_eq!(pq.remove(), Some((200, 6)));
    assert_eq!(pq.size(), 1);

    assert_eq!(pq.remove(), Some((500, 8)));
    assert_eq!(pq.size(), 0);

    assert!(pq.is_empty());
    assert_eq!(pq.remove(), None);
}

#[test]
fn clear() {
    let mut pq = seed(IndexPriorityQueue::new(|a, b| a > b), [(100, 4), (200, 6), (300, 4), (400, 3), (500, 8)]);

    pq.clear();

    assert!(pq.is_empty());
    assert_eq!(pq.size(), 0);
    assert_eq!(pq.peek(), None);
}

#[test]
fn into_iter() {
    let pq = seed(IndexPriorityQueue::new(|a, b| a > b), [(100, 4), (200, 6), (300, 4), (400, 3), (500, 8)]);

    let collected: Vec<(usize, i32)> = pq.into_iter().collect();
    assert_eq!(collected, vec![(400, 3), (100, 4), (300, 4), (200, 6), (500, 8)]);
}

#[test]
fn max_queue() {
    let pq = seed(IndexPriorityQueue::new_max(), [(100, 4), (200, 6), (300, 4), (400, 3), (500, 8)]);

    let collected: Vec<(usize, i32)> = pq.into();
    assert_eq!(collected, vec![(500, 8), (200, 6), (100, 4), (300, 4), (400, 3)]);
}

#[test]
fn min_queue() {
    let pq = seed(IndexPriorityQueue::new_min(), [(100, 4), (200, 6), (300, 4), (400, 3), (500, 8)]);

    let collected: Vec<(usize, i32)> = pq.into();
    assert_eq!(collected, vec![(400, 3), (100, 4), (300, 4), (200, 6), (500, 8)]);
}

#[test]
fn custom_struct() {
    #[derive(Eq, PartialEq, Debug)]
    struct Tester {
        val: i32,
    }
    let mut pq = IndexPriorityQueue::new(|a: &Tester, b: &Tester| a.val > b.val);
    [(100, 4), (200, 6), (300, 4), (400, 3), (500, 8)]
        .into_iter()
        .for_each(|(k, i)| pq.insert((k, Tester { val: i })));

    assert_eq!(pq.remove().unwrap(), (400, Tester { val: 3 }));
    assert_eq!(pq.remove().unwrap(), (100, Tester { val: 4 }));
    assert_eq!(pq.remove().unwrap(), (300, Tester { val: 4 }));
    assert_eq!(pq.remove().unwrap(), (200, Tester { val: 6 }));
    assert_eq!(pq.remove().unwrap(), (500, Tester { val: 8 }));
}
