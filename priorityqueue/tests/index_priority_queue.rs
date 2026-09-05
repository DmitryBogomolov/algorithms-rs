use priorityqueue::IndexPriorityQueue;
use std::hash::Hash;

#[test]
fn empty() {
    let pq = IndexPriorityQueue::<(), (), _>::new_ord(|_, _| false);
    assert!(pq.is_empty());
    assert_eq!(pq.len(), 0);
    assert_eq!(pq.peek(), None);
}

#[test]
fn insert() {
    let mut pq = IndexPriorityQueue::new_ord(|a, b| a < b);

    assert_eq!(pq.insert(('a', 4)), None);
    assert!(!pq.is_empty());
    assert_eq!(pq.len(), 1);
    assert_eq!(pq.peek(), Some((&'a', &4)));
    assert_eq!(pq.peek_idx(&'a'), Some((&'a', &4)));

    assert_eq!(pq.insert(('b', 7)), None);
    assert_eq!(pq.len(), 2);
    assert_eq!(pq.peek(), Some((&'b', &7)));
    assert_eq!(pq.peek_idx(&'b'), Some((&'b', &7)));
    assert_eq!(pq.peek_idx(&'a'), Some((&'a', &4)));

    assert_eq!(pq.insert(('c', 2)), None);
    assert_eq!(pq.len(), 3);
    assert_eq!(pq.peek(), Some((&'b', &7)));
    assert_eq!(pq.peek_idx(&'c'), Some((&'c', &2)));
    assert_eq!(pq.peek_idx(&'b'), Some((&'b', &7)));

    assert_eq!(pq.insert(('d', 7)), None);
    assert_eq!(pq.len(), 4);
    assert_eq!(pq.peek(), Some((&'b', &7)));
    assert_eq!(pq.peek_idx(&'b'), Some((&'b', &7)));
    assert_eq!(pq.peek_idx(&'c'), Some((&'c', &2)));

    assert_eq!(pq.insert(('e', 9)), None);
    assert_eq!(pq.len(), 5);
    assert_eq!(pq.peek(), Some((&'e', &9)));
    assert_eq!(pq.peek_idx(&'e'), Some((&'e', &9)));
    assert_eq!(pq.peek_idx(&'a'), Some((&'a', &4)));
}

#[test]
fn insert_update() {
    let mut pq = IndexPriorityQueue::new_ord(|a, b| a < b);

    pq.insert(('a', 4));
    pq.insert(('b', 7));
    pq.insert(('c', 2));
    assert_eq!(pq.len(), 3);

    assert_eq!(pq.insert(('c', 1)), Some(('c', 2)));
    assert_eq!(pq.len(), 3);
    assert_eq!(pq.peek(), Some((&'b', &7)));

    assert_eq!(pq.insert(('b', 8)), Some(('b', 7)));
    assert_eq!(pq.len(), 3);
    assert_eq!(pq.peek(), Some((&'b', &8)));

    assert_eq!(pq.insert(('a', 9)), Some(('a', 4)));
    assert_eq!(pq.len(), 3);
    assert_eq!(pq.peek(), Some((&'a', &9)));
}

fn make<K, T, F, I>(is_ord: F, items: I) -> IndexPriorityQueue<K, T, F>
where
    K: Hash + Eq + Clone,
    F: FnMut(&T, &T) -> bool,
    I: IntoIterator<Item = (K, T)>,
{
    let mut pq = IndexPriorityQueue::new_ord(is_ord);
    items.into_iter().for_each(|i| {
        pq.insert(i);
    });
    pq
}

#[test]
fn remove() {
    let mut pq = make(
        |a, b| a > b,
        [('a', 4), ('b', 6), ('c', 5), ('d', 3), ('e', 8)],
    );

    assert_eq!(pq.len(), 5);
    assert_eq!(pq.peek(), Some((&'d', &3)));

    assert_eq!(pq.remove(), Some(('d', 3)));
    assert_eq!(pq.len(), 4);
    assert_eq!(pq.peek_idx(&'d'), None);

    assert_eq!(pq.remove(), Some(('a', 4)));
    assert_eq!(pq.len(), 3);
    assert_eq!(pq.peek_idx(&'a'), None);

    assert_eq!(pq.remove(), Some(('c', 5)));
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
fn remove_idx() {
    let mut pq = make(
        |a, b| a > b,
        [('a', 4), ('b', 6), ('c', 5), ('d', 3), ('e', 8)],
    );

    assert_eq!(pq.remove_idx(&'e'), Some(('e', 8)));
    assert_eq!(pq.remove_idx(&'a'), Some(('a', 4)));
    assert_eq!(pq.remove_idx(&'f'), None);
    assert_eq!(pq.remove_idx(&'d'), Some(('d', 3)));
    assert_eq!(pq.remove_idx(&'b'), Some(('b', 6)));
    assert_eq!(pq.remove_idx(&'c'), Some(('c', 5)));
    assert_eq!(pq.remove_idx(&'b'), None);
}

#[test]
fn clear() {
    let mut pq = make(
        |a, b| a > b,
        [('a', 4), ('b', 6), ('c', 5), ('d', 3), ('e', 8)],
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
        [('a', 4), ('b', 6), ('c', 5), ('d', 3), ('e', 8)],
    );

    let vec: Vec<_> = pq.into();
    assert_eq!(vec, [('d', 3), ('a', 4), ('c', 5), ('b', 6), ('e', 8)]);
}

#[test]
fn into_iter() {
    let pq = make(
        |a, b| a > b,
        [('a', 4), ('b', 6), ('c', 5), ('d', 3), ('e', 8)],
    );

    let vec: Vec<_> = pq.into_iter().collect();
    assert_eq!(vec, [('d', 3), ('a', 4), ('c', 5), ('b', 6), ('e', 8)]);
}

#[test]
fn drain_full() {
    let mut pq = make(
        |a, b| a > b,
        [('a', 4), ('b', 6), ('c', 5), ('d', 3), ('e', 8)],
    );

    let vec: Vec<_> = pq.drain().collect();
    assert_eq!(vec, [('d', 3), ('a', 4), ('c', 5), ('b', 6), ('e', 8)]);
    assert_eq!(pq.peek(), None);
    assert!(pq.is_empty());
}

#[test]
fn drain_partial() {
    let mut pq = make(
        |a, b| a > b,
        [('a', 4), ('b', 6), ('c', 5), ('d', 3), ('e', 8)],
    );

    let collected: Vec<_> = pq.drain().take(2).collect();
    assert_eq!(collected, [('d', 3), ('a', 4)]);
    assert!(pq.is_empty());
}

#[test]
fn drain_empty() {
    let mut pq: IndexPriorityQueue<(), (), _> = IndexPriorityQueue::new_ord(|a, b| a < b);

    let vec: Vec<_> = pq.drain().collect();
    assert_eq!(vec, []);
}

#[test]
fn queue_of_ordered() {
    let mut pq = IndexPriorityQueue::new();
    [('a', 4), ('b', 6), ('c', 5), ('d', 3), ('e', 8)]
        .into_iter()
        .for_each(|t| {
            pq.insert(t);
        });

    let vec: Vec<_> = pq.into();
    assert_eq!(vec, [('e', 8), ('b', 6), ('c', 5), ('a', 4), ('d', 3)]);
}

#[test]
fn custom_struct() {
    #[derive(Eq, PartialEq, Debug)]
    struct Tester {
        val: i32,
    }

    let mut pq = make(
        |a, b| a.val > b.val,
        [("a", 4), ("b", 6), ("c", 5), ("d", 3), ("e", 8)]
            .map(|(k, v)| (k.to_owned(), Tester { val: v })),
    );

    assert_eq!(
        pq.peek_idx("a"),
        Some((&"a".to_owned(), &Tester { val: 4 }))
    );
    assert_eq!(
        pq.peek_idx("b"),
        Some((&"b".to_owned(), &Tester { val: 6 }))
    );
    assert_eq!(
        pq.peek_idx("c"),
        Some((&"c".to_owned(), &Tester { val: 5 }))
    );
    assert_eq!(
        pq.peek_idx("d"),
        Some((&"d".to_owned(), &Tester { val: 3 }))
    );
    assert_eq!(
        pq.peek_idx("e"),
        Some((&"e".to_owned(), &Tester { val: 8 }))
    );
    assert_eq!(pq.remove(), Some(("d".to_owned(), Tester { val: 3 })));
    assert_eq!(pq.remove(), Some(("a".to_owned(), Tester { val: 4 })));
    assert_eq!(pq.remove(), Some(("c".to_owned(), Tester { val: 5 })));
    assert_eq!(pq.remove(), Some(("b".to_owned(), Tester { val: 6 })));
    assert_eq!(pq.remove(), Some(("e".to_owned(), Tester { val: 8 })));
}

#[test]
fn test_many() {
    let mut pq = IndexPriorityQueue::new_ord(|a, b| a < b);

    for i in 0..400 {
        assert_eq!(pq.insert(((i + 1).to_string(), i + 1)), None);
    }
    assert_eq!(pq.len(), 400);
    assert_eq!(pq.peek(), Some((&"400".to_owned(), &400)));
    for i in 0..400 {
        let k = (i + 1).to_string();
        assert_eq!(pq.peek_idx(&k), Some((&k, &(i + 1))));
    }

    for i in (300..400).rev() {
        assert_eq!(pq.remove(), Some(((i + 1).to_string(), i + 1)));
    }
    assert_eq!(pq.len(), 300);
    assert_eq!(pq.peek(), Some((&"300".to_owned(), &300)));

    for i in 400..1200 {
        assert_eq!(pq.insert(((i + 1).to_string(), i + 1)), None);
    }
    assert_eq!(pq.len(), 1100);
    assert_eq!(pq.peek(), Some((&"1200".to_owned(), &1200)));
    for i in 400..1200 {
        let k = (i + 1).to_string();
        assert_eq!(pq.peek_idx(&k), Some((&k, &(i + 1))));
    }

    for i in (800..1200).rev() {
        assert_eq!(pq.remove(), Some(((i + 1).to_string(), i + 1)));
    }
    assert_eq!(pq.len(), 700);
    assert_eq!(pq.peek(), Some((&"800".to_owned(), &800)));

    let vec: Vec<_> = pq.into();
    let expected: Vec<_> = (0..300)
        .chain(400..800)
        .rev()
        .map(|t| ((t + 1).to_string(), t + 1))
        .collect();
    assert_eq!(vec, expected);
}

#[test]
fn indexing() {
    let pq = make(
        |a, b| a > b,
        [
            ("a".to_owned(), 4),
            ("b".to_owned(), 6),
            ("c".to_owned(), 5),
            ("d".to_owned(), 3),
            ("e".to_owned(), 8),
        ],
    );

    assert_eq!(pq["a"], 4);
    assert_eq!(pq["b"], 6);
    assert_eq!(pq["c"], 5);
    assert_eq!(pq["d"], 3);
    assert_eq!(pq["e"], 8);
}

#[test]
fn clone() {
    let pq = {
        let mut pq = make(
            |a, b| a > b,
            [('a', 4), ('b', 6), ('c', 5), ('d', 3), ('e', 8)],
        );
        let ret = pq.clone();
        pq.clear();
        ret
    };

    let vec: Vec<_> = pq.into();
    assert_eq!(vec, [('d', 3), ('a', 4), ('c', 5), ('b', 6), ('e', 8)]);
}

#[test]
fn from_iterator_with_func() {
    let pq = IndexPriorityQueue::from_iter_ord(
        |a, b| a < b,
        [
            ('a', 4),
            ('b', 6),
            ('c', 5),
            ('d', 2),
            ('e', 3),
            ('f', 9),
            ('g', 8),
        ],
    );

    let vec: Vec<_> = pq.into();
    assert_eq!(
        vec,
        [
            ('f', 9),
            ('g', 8),
            ('b', 6),
            ('c', 5),
            ('a', 4),
            ('e', 3),
            ('d', 2)
        ]
    );
}

#[test]
fn from_iterator() {
    let pq: IndexPriorityQueue<_, _, _> = [
        ('a', 4),
        ('b', 6),
        ('c', 5),
        ('d', 2),
        ('e', 3),
        ('f', 9),
        ('g', 8),
    ]
    .into_iter()
    .collect();

    let vec: Vec<_> = pq.into();
    assert_eq!(
        vec,
        [
            ('f', 9),
            ('g', 8),
            ('b', 6),
            ('c', 5),
            ('a', 4),
            ('e', 3),
            ('d', 2)
        ]
    );
}

#[test]
fn from_array_with_func() {
    let pq = IndexPriorityQueue::from_arr_ord(
        |a, b| a < b,
        [
            ('a', 4),
            ('b', 6),
            ('c', 5),
            ('d', 2),
            ('e', 3),
            ('f', 9),
            ('g', 8),
        ],
    );

    let vec: Vec<_> = pq.into();
    assert_eq!(
        vec,
        [
            ('f', 9),
            ('g', 8),
            ('b', 6),
            ('c', 5),
            ('a', 4),
            ('e', 3),
            ('d', 2)
        ]
    );
}

#[test]
fn from_array() {
    let pq: IndexPriorityQueue<_, _, _> = [
        ('a', 4),
        ('b', 6),
        ('c', 5),
        ('d', 2),
        ('e', 3),
        ('f', 9),
        ('g', 8),
    ]
    .into();

    let vec: Vec<_> = pq.into();
    assert_eq!(
        vec,
        [
            ('f', 9),
            ('g', 8),
            ('b', 6),
            ('c', 5),
            ('a', 4),
            ('e', 3),
            ('d', 2)
        ]
    );
}
