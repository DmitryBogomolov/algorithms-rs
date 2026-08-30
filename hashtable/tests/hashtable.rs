use std::hash::RandomState;
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

    assert_eq!(table.insert("11".to_string(), 11), None);
    assert!(!table.is_empty());
    assert_eq!(table.len(), 1);
    assert_eq!(table.get("11"), Some(&11));
    assert_eq!(table.get("11_"), None);
    assert_eq!(table.get_kv("11"), Some((&"11".to_string(), &11)));
    assert_eq!(table.get_kv("11_"), None);
}

#[test]
fn remove() {
    let mut table = HashTable::new();
    table.insert("11".to_string(), 11);

    assert_eq!(table.remove("11"), Some(("11".to_string(), 11)));
    assert!(table.is_empty());
    assert_eq!(table.len(), 0);
    assert_eq!(table.remove("11"), None);
    assert_eq!(table.get("11"), None);
}

#[test]
fn clear() {
    let mut table = HashTable::new();
    table.insert(1, 'a');
    table.insert(2, 'b');

    table.clear();

    assert!(table.is_empty());
    assert_eq!(table.len(), 0);
}

#[test]
fn mutate() {
    let mut table = HashTable::new();
    table.insert("11".to_string(), 11);

    *table.get_mut("11").unwrap() += 1;
    assert_eq!(table.get("11"), Some(&12));

    *table.get_kv_mut("11").unwrap().1 += 2;
    assert_eq!(table.get_kv("11"), Some((&"11".to_string(), &14)));
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
    let mut table = HashTable::with_hasher(RandomState::new());

    assert!(table.is_empty());
    assert_eq!(table.len(), 0);

    table.insert("1", 1);
    assert_eq!(table.len(), 1);
}
