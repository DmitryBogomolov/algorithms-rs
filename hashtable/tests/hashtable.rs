use hashtable::HashTable;

#[test]
fn empty() {
    let table: HashTable<(), ()> = HashTable::new();
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
    for i in 0..400 {
        assert_eq!(table.insert(1000 + i, i), None);
    }
    for i in (0..400).step_by(2) {
        assert_eq!(table.remove(&(1000 + i)), Some((1000 + i, i)));
    }
    for i in 400..800 {
        assert_eq!(table.insert(1000 + i, i), None);
    }
    for i in (400..800).step_by(2) {
        assert_eq!(table.remove(&(1000 + i)), Some((1000 + i, i)));
    }
}

#[test]
fn from_iterator() {
    let tree: HashTable<i32, char> = [(1, 'a'), (2, 'b'), (3, 'c')].into_iter().collect();

    assert_eq!(tree.len(), 3);
    assert_eq!(tree.get(&1), Some(&'a'));
    assert_eq!(tree.get(&2), Some(&'b'));
    assert_eq!(tree.get(&3), Some(&'c'));
}

#[test]
fn from_array() {
    let tree: HashTable<_, _> = [(1, 'a'), (2, 'b'), (3, 'c')].into();

    assert_eq!(tree.len(), 3);
    assert_eq!(tree.get(&1), Some(&'a'));
    assert_eq!(tree.get(&2), Some(&'b'));
    assert_eq!(tree.get(&3), Some(&'c'));
}


#[test]
fn indexing() {
    let mut tree: HashTable<_, _> = [
        (2, "b".to_owned()),
        (3, "c".to_owned()),
        (1, "a".to_owned()),
    ]
    .into();

    assert_eq!(tree[&1], "a");
    assert_eq!(tree[&2], "b");
    assert_eq!(tree[&3], "c");

    tree[&1] = "A".to_owned();
    tree[&2] = "B".to_owned();
    tree[&3] = "C".to_owned();

    assert_eq!(tree[&1], "A");
    assert_eq!(tree[&2], "B");
    assert_eq!(tree[&3], "C");
}
