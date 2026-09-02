use priorityqueue::IndexPriorityQueue;
use std::hash::Hash;

#[test]
fn empty() {
    let pq = IndexPriorityQueue::<(), (), _>::new(|_, _| false);
    assert!(pq.is_empty());
    assert_eq!(pq.len(), 0);
    assert_eq!(pq.peek(), None);
}

#[test]
fn insert() {
    let mut pq = IndexPriorityQueue::new(|a, b| a < b);

    pq.insert(('a', 4));
    assert!(!pq.is_empty());
    assert_eq!(pq.len(), 1);
    assert_eq!(pq.peek(), Some(&('a', 4)));
    assert_eq!(pq.peek_idx(&'a'), Some(&('a', 4)));

    pq.insert(('b', 7));
    assert_eq!(pq.len(), 2);
    assert_eq!(pq.peek(), Some(&('b', 7)));
    assert_eq!(pq.peek_idx(&'b'), Some(&('b', 7)));
    assert_eq!(pq.peek_idx(&'a'), Some(&('a', 4)));

    pq.insert(('c', 2));
    assert_eq!(pq.len(), 3);
    assert_eq!(pq.peek(), Some(&('b', 7)));
    assert_eq!(pq.peek_idx(&'c'), Some(&('c', 2)));
    assert_eq!(pq.peek_idx(&'b'), Some(&('b', 7)));

    pq.insert(('d', 7));
    assert_eq!(pq.len(), 4);
    assert_eq!(pq.peek(), Some(&('b', 7)));
    assert_eq!(pq.peek_idx(&'b'), Some(&('b', 7)));
    assert_eq!(pq.peek_idx(&'c'), Some(&('c', 2)));

    pq.insert(('e', 9));
    assert_eq!(pq.len(), 5);
    assert_eq!(pq.peek(), Some(&('e', 9)));
    assert_eq!(pq.peek_idx(&'e'), Some(&('e', 9)));
    assert_eq!(pq.peek_idx(&'a'), Some(&('a', 4)));
}

#[test]
fn insert_update() {
    let mut pq = IndexPriorityQueue::new(|a, b| a < b);

    pq.insert(('a', 4));
    pq.insert(('b', 7));
    pq.insert(('c', 2));
    assert_eq!(pq.len(), 3);

    pq.insert(('c', 1));
    assert_eq!(pq.len(), 3);
    assert_eq!(pq.peek(), Some(&('b', 7)));

    pq.insert(('b', 8));
    assert_eq!(pq.len(), 3);
    assert_eq!(pq.peek(), Some(&('b', 8)));

    pq.insert(('a', 9));
    assert_eq!(pq.len(), 3);
    assert_eq!(pq.peek(), Some(&('a', 9)));
}

fn make<K, T, F, I>(is_ord: F, items: I) -> IndexPriorityQueue<K, T, F>
where
    K: Hash + Eq + Clone,
    F: FnMut(&T, &T) -> bool,
    I: IntoIterator<Item = (K, T)>,
{
    let mut pq = IndexPriorityQueue::new(is_ord);
    items.into_iter().for_each(|i| pq.insert(i));
    pq
}

#[test]
fn remove() {
    let mut pq = make(
        |a, b| a > b,
        [('a', 4), ('b', 6), ('c', 4), ('d', 3), ('e', 8)],
    );

    assert_eq!(pq.len(), 5);
    assert_eq!(pq.peek(), Some(&('d', 3)));

    assert_eq!(pq.remove(), Some(('d', 3)));
    assert_eq!(pq.len(), 4);
    assert_eq!(pq.peek_idx(&'d'), None);

    assert_eq!(pq.remove(), Some(('a', 4)));
    assert_eq!(pq.len(), 3);
    assert_eq!(pq.peek_idx(&'a'), None);

    assert_eq!(pq.remove(), Some(('c', 4)));
    assert_eq!(pq.len(), 2);
    assert_eq!(pq.peek_idx(&'c'), None);

    assert_eq!(pq.remove(), Some(('b', 6)));
    assert_eq!(pq.len(), 1);
    assert_eq!(pq.peek_idx(&'b'), None);

    assert_eq!(pq.remove(), Some(('e', 8)));
    assert_eq!(pq.len(), 0);
    assert_eq!(pq.peek_idx(&'e'), None);

    assert!(pq.is_empty());
    assert_eq!(pq.remove(), None);
}

#[test]
fn clear() {
    let mut pq = make(
        |a, b| a > b,
        [('a', 4), ('b', 6), ('c', 4), ('d', 3), ('e', 8)],
    );

    pq.clear();

    assert!(pq.is_empty());
    assert_eq!(pq.len(), 0);
    assert_eq!(pq.peek(), None);
}

#[test]
fn into_vec() {
    let pq = make(
        |a, b| a > b,
        [('a', 4), ('b', 6), ('c', 4), ('d', 3), ('e', 8)],
    );

    let vec: Vec<_> = pq.into();
    assert_eq!(vec, [('d', 3), ('a', 4), ('c', 4), ('b', 6), ('e', 8)]);
}

#[test]
fn into_iter() {
    let pq = make(
        |a, b| a > b,
        [('a', 4), ('b', 6), ('c', 4), ('d', 3), ('e', 8)],
    );

    let vec: Vec<_> = pq.into_iter().collect();
    assert_eq!(vec, [('d', 3), ('a', 4), ('c', 4), ('b', 6), ('e', 8)]);
}

#[test]
fn drain_full() {
    let mut pq = make(
        |a, b| a > b,
        [('a', 4), ('b', 6), ('c', 4), ('d', 3), ('e', 8)],
    );

    let vec: Vec<(char, i32)> = pq.drain().collect();
    assert_eq!(vec, [('d', 3), ('a', 4), ('c', 4), ('b', 6), ('e', 8)]);
    assert!(pq.is_empty());
}

#[test]
fn drain_partial() {
    let mut pq = make(
        |a, b| a > b,
        [('a', 4), ('b', 6), ('c', 4), ('d', 3), ('e', 8)],
    );

    let collected: Vec<_> = pq.drain().take(2).collect();
    assert_eq!(collected, [('d', 3), ('a', 4)]);
    assert_eq!(pq.len(), 3);
    assert_eq!(pq.remove(), Some(('c', 4)));
    assert_eq!(pq.remove(), Some(('b', 6)));
    assert_eq!(pq.remove(), Some(('e', 8)));
}

#[test]
fn drain_empty() {
    let mut pq: IndexPriorityQueue<(), (), _> = IndexPriorityQueue::new(|a, b| a < b);

    let vec: Vec<_> = pq.drain().collect();
    assert_eq!(vec, []);
}

#[test]
fn max_queue() {
    let mut pq = IndexPriorityQueue::new_max();
    [('a', 4), ('b', 6), ('c', 4), ('d', 3), ('e', 8)]
        .into_iter()
        .for_each(|t| pq.insert(t));

    let vec: Vec<_> = pq.into();
    assert_eq!(vec, [('e', 8), ('b', 6), ('a', 4), ('c', 4), ('d', 3)]);
}

#[test]
fn min_queue() {
    let mut pq = IndexPriorityQueue::new_min();
    [('a', 4), ('b', 6), ('c', 4), ('d', 3), ('e', 8)]
        .into_iter()
        .for_each(|t| pq.insert(t));

    let vec: Vec<_> = pq.into();
    assert_eq!(vec, [('d', 3), ('a', 4), ('c', 4), ('b', 6), ('e', 8)]);
}

#[test]
fn custom_struct() {
    #[derive(Eq, PartialEq, Debug)]
    struct Tester {
        val: i32,
    }
    let mut pq = IndexPriorityQueue::new(|a: &Tester, b: &Tester| a.val > b.val);
    [('a', 4), ('b', 6), ('c', 4), ('d', 3), ('e', 8)]
        .into_iter()
        .for_each(|(k, i)| pq.insert((k, Tester { val: i })));

    assert_eq!(pq.remove().unwrap(), ('d', Tester { val: 3 }));
    assert_eq!(pq.remove().unwrap(), ('a', Tester { val: 4 }));
    assert_eq!(pq.remove().unwrap(), ('c', Tester { val: 4 }));
    assert_eq!(pq.remove().unwrap(), ('b', Tester { val: 6 }));
    assert_eq!(pq.remove().unwrap(), ('e', Tester { val: 8 }));
}

#[test]
fn remove_idx() {
    let mut pq = make(
        |a, b| a > b,
        [('a', 4), ('b', 6), ('c', 4), ('d', 3), ('e', 8)],
    );

    assert_eq!(pq.remove_idx(&'e'), Some(('e', 8)));
    assert_eq!(pq.remove_idx(&'a'), Some(('a', 4)));
    assert_eq!(pq.remove_idx(&'f'), None);
    assert_eq!(pq.remove_idx(&'d'), Some(('d', 3)));
    assert_eq!(pq.remove_idx(&'b'), Some(('b', 6)));
    assert_eq!(pq.remove_idx(&'c'), Some(('c', 4)));
    assert_eq!(pq.remove_idx(&'b'), None);
}

#[test]
fn string_key() {
    let mut pq = IndexPriorityQueue::new(|a, b| a > b);

    pq.insert(("a1".to_string(), 4));
    pq.insert(("b2".to_string(), 7));
    pq.insert(("c3".to_string(), 2));

    assert_eq!(pq.len(), 3);
    assert_eq!(pq.peek(), Some(&("c3".to_string(), 2)));
    assert_eq!(pq.peek_idx("a1"), Some(&("a1".to_string(), 4)));

    assert_eq!(pq.remove_idx("b2"), Some(("b2".to_string(), 7)));
    assert_eq!(pq.len(), 2);
    assert_eq!(pq.peek_idx("b2"), None);

    assert_eq!(pq.remove(), Some(("c3".to_string(), 2)));
    assert_eq!(pq.remove(), Some(("a1".to_string(), 4)));
    assert!(pq.is_empty());
}
