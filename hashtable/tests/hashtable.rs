use hashtable::HashTable;

#[test]
fn empty() {
    let table: HashTable<(), (), _> = HashTable::new();
    assert!(table.is_empty());
    assert_eq!(table.len(), 0);
}

#[test]
fn insert() {
    let mut table = HashTable::new();

    assert_eq!(table.insert("11".to_owned(), 11), None);
    assert!(!table.is_empty());
    assert_eq!(table.len(), 1);

    assert_eq!(table.get("11"), Some(&11));
    assert_eq!(table.get("11_"), None);
    assert_eq!(table.get_kv("11"), Some((&"11".to_owned(), &11)));
    assert_eq!(table.get_kv("11_"), None);

    assert_eq!(
        table.insert("11".to_owned(), 12),
        Some(("11".to_owned(), 11))
    );
    assert_eq!(table.len(), 1);

    assert_eq!(table.insert("12".to_owned(), 12), None);
    assert_eq!(table.len(), 2);

    assert_eq!(
        table.insert("12".to_owned(), 11),
        Some(("12".to_owned(), 12))
    );
    assert_eq!(table.len(), 2);
}

#[test]
fn remove() {
    let mut table = HashTable::new();
    table.insert("11".to_owned(), 11);
    table.insert("12".to_owned(), 12);

    assert_eq!(table.remove("11"), Some(("11".to_owned(), 11)));
    assert_eq!(table.len(), 1);

    assert_eq!(table.remove("11"), None);
    assert_eq!(table.get("11"), None);
    assert_eq!(table.len(), 1);

    assert_eq!(table.remove("12"), Some(("12".to_owned(), 12)));
    assert_eq!(table.len(), 0);

    assert_eq!(table.remove("12"), None);
    assert_eq!(table.get("12"), None);
    assert_eq!(table.len(), 0);
    assert!(table.is_empty());
}

#[test]
fn clear() {
    let mut table = HashTable::new();
    table.insert("1".to_owned(), 1);
    table.insert("2".to_owned(), 2);
    table.insert("3".to_owned(), 3);

    table.clear();

    assert!(table.is_empty());
    assert_eq!(table.len(), 0);
}

#[test]
fn mutate() {
    let mut table = HashTable::new();
    table.insert("11".to_owned(), 11);

    *table.get_mut("11").unwrap() += 1;
    assert_eq!(table.get("11"), Some(&12));

    *table.get_kv_mut("11").unwrap().1 += 2;
    assert_eq!(table.get_kv("11"), Some((&"11".to_owned(), &14)));
}

#[test]
fn test_many() {
    let mut table = HashTable::new();
    let r1 = 0..400;
    let r2 = r1.clone().rev().filter(|i| i % 4 != 0);
    let r3 = 400..1200;
    let r4 = r3.clone().rev().filter(|i| i % 12 != 0);

    for i in r1.clone() {
        assert_eq!(table.insert(1000 + i, i), None);
    }
    for i in r1.clone() {
        assert_eq!(table.get(&(1000 + i)), Some(&i));
    }
    for i in r2.clone() {
        assert_eq!(table.remove(&(1000 + i)), Some((1000 + i, i)));
    }
    for i in r2.clone() {
        assert_eq!(table.get(&(1000 + i)), None);
    }
    for i in r3.clone() {
        assert_eq!(table.insert(1000 + i, i), None);
    }
    for i in r3.clone() {
        assert_eq!(table.get(&(1000 + i)), Some(&i));
    }
    for i in r4.clone() {
        assert_eq!(table.remove(&(1000 + i)), Some((1000 + i, i)));
    }
    for i in r4.clone() {
        assert_eq!(table.get(&(1000 + i)), None);
    }
}

#[test]
fn from_iterator() {
    let table: HashTable<_, _, _> = [(1, 'a'), (2, 'b'), (3, 'c')].into_iter().collect();

    assert_eq!(table.len(), 3);
    assert_eq!(table.get(&1), Some(&'a'));
    assert_eq!(table.get(&2), Some(&'b'));
    assert_eq!(table.get(&3), Some(&'c'));
}

#[test]
fn from_array() {
    let table: HashTable<_, _, _> = [(1, 'a'), (2, 'b'), (3, 'c')].into();

    assert_eq!(table.len(), 3);
    assert_eq!(table.get(&1), Some(&'a'));
    assert_eq!(table.get(&2), Some(&'b'));
    assert_eq!(table.get(&3), Some(&'c'));
}

#[test]
fn indexing() {
    let mut table: HashTable<_, _, _> = [
        (2, "b".to_owned()),
        (3, "c".to_owned()),
        (1, "a".to_owned()),
    ]
    .into();

    assert_eq!(table[&1], "a");
    assert_eq!(table[&2], "b");
    assert_eq!(table[&3], "c");

    table[&1] = "A".to_owned();
    table[&2] = "B".to_owned();
    table[&3] = "C".to_owned();

    assert_eq!(table[&1], "A");
    assert_eq!(table[&2], "B");
    assert_eq!(table[&3], "C");
}

#[test]
fn iter() {
    let table: HashTable<_, _, _> = [(2, 'b'), (3, 'c'), (1, 'a')].into();

    let mut items = vec![];
    for item in table.iter() {
        items.push(item);
    }

    items.sort_by_key(|item| item.0);
    assert_eq!(items, [(&1, &'a'), (&2, &'b'), (&3, &'c')]);
}

#[test]
fn iter_mut() {
    let mut table: HashTable<_, _, _> = [(2, 'b'), (3, 'c'), (1, 'a')].into();

    let mut items = vec![];
    for item in table.iter_mut() {
        *item.1 = '0';
        items.push(item);
    }

    items.sort_by_key(|item| item.0);
    assert_eq!(items, [(&1, &mut '0'), (&2, &mut '0'), (&3, &mut '0')]);
    assert_eq!(table.get(&1), Some(&'0'));
    assert_eq!(table.get(&2), Some(&'0'));
    assert_eq!(table.get(&3), Some(&'0'));
}

#[test]
fn drain() {
    let mut table: HashTable<_, _, _> = [(2, 'b'), (3, 'c'), (1, 'a')].into();

    let mut items: Vec<_> = table.drain().collect();

    assert_eq!(table.len(), 0);
    assert!(table.is_empty());
    items.sort_by_key(|item| item.0);
    assert_eq!(items, [(1, 'a'), (2, 'b'), (3, 'c')]);

    assert_eq!(table.insert(2, '0'), None);
    assert_eq!(table.get(&2), Some(&'0'));
}

#[test]
fn into_iter() {
    let table: HashTable<_, _, _> = [(2, 'b'), (3, 'c'), (1, 'a')].into();

    let mut items = vec![];
    for item in table {
        items.push(item);
    }

    items.sort_by_key(|item| item.0);
    assert_eq!(items, [(1, 'a'), (2, 'b'), (3, 'c')]);
}

#[test]
fn into_iter_ref() {
    let table: HashTable<_, _, _> = [(2, 'b'), (3, 'c'), (1, 'a')].into();

    let mut items = vec![];
    for item in &table {
        items.push(item);
    }

    items.sort_by_key(|item| item.0);
    assert_eq!(items, [(&1, &'a'), (&2, &'b'), (&3, &'c')]);
}

#[test]
fn into_iter_mut() {
    let mut table: HashTable<_, _, _> = [(2, 'b'), (3, 'c'), (1, 'a')].into();

    let mut items = vec![];
    for item in &mut table {
        *item.1 = '0';
        items.push(item);
    }

    items.sort_by_key(|item| item.0);
    assert_eq!(items, [(&1, &mut '0'), (&2, &mut '0'), (&3, &mut '0')]);
    assert_eq!(table.get(&1), Some(&'0'));
    assert_eq!(table.get(&2), Some(&'0'));
    assert_eq!(table.get(&3), Some(&'0'));
}

#[test]
fn clone() {
    let mut table: HashTable<_, _, _> = [(1, 'a'), (2, 'b'), (3, 'c')].into();
    let clone = table.clone();
    table.clear();

    assert_eq!(clone.len(), 3);
    assert_eq!(clone[&1], 'a');
    assert_eq!(clone[&2], 'b');
    assert_eq!(clone[&3], 'c');
}

#[test]
fn with_hasher() {
    let mut table = HashTable::with_hasher(std::hash::RandomState::new());

    assert!(table.is_empty());
    assert_eq!(table.len(), 0);

    table.insert("1", 1);
    assert_eq!(table.len(), 1);
}

#[derive(Default)]
struct DumbHasher;

impl std::hash::Hasher for DumbHasher {
    fn write(&mut self, _bytes: &[u8]) {}

    fn finish(&self) -> u64 {
        101
    }
}

type DumbBuildHasher = std::hash::BuildHasherDefault<DumbHasher>;

#[test]
fn dumb_hasher() {
    let mut table = HashTable::with_hasher(DumbBuildHasher::new());
    let r = 0..400;

    for i in r.clone() {
        assert_eq!(table.insert(i.to_string(), i + 1000), None);
    }
    assert_eq!(table.len(), 400);
    for i in r.clone().rev() {
        assert_eq!(table.get(&i.to_string()), Some(&(1000 + i)));
    }
    for i in r.clone() {
        assert_eq!(
            table.remove(&i.to_string()),
            Some((i.to_string(), i + 1000))
        );
    }
    assert_eq!(table.len(), 0);
}
