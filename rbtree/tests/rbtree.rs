use rbtree::RBTree;

#[test]
fn empty() {
    let tree: RBTree<(), ()> = RBTree::new();
    assert!(tree.is_empty());
    assert_eq!(tree.len(), 0);
}

#[test]
fn insert() {
    let mut tree = RBTree::new();

    assert_eq!(tree.insert("11".to_owned(), 11), None);
    assert!(!tree.is_empty());
    assert_eq!(tree.len(), 1);

    assert_eq!(tree.get("11"), Some(&11));
    assert_eq!(tree.get("11_"), None);
    assert_eq!(tree.get_key_val("11"), Some((&"11".to_owned(), &11)));
    assert_eq!(tree.get_key_val("11_"), None);

    assert_eq!(
        tree.insert("11".to_owned(), 12),
        Some(("11".to_owned(), 11))
    );
    assert_eq!(tree.len(), 1);

    assert_eq!(tree.insert("12".to_owned(), 12), None);
    assert_eq!(tree.len(), 2);

    assert_eq!(
        tree.insert("12".to_owned(), 11),
        Some(("12".to_owned(), 12))
    );
    assert_eq!(tree.len(), 2);
}

#[test]
fn remove() {
    let mut tree = RBTree::new();
    tree.insert("11".to_owned(), 11);
    tree.insert("12".to_owned(), 12);

    assert_eq!(tree.remove("11"), Some(("11".to_owned(), 11)));
    assert_eq!(tree.len(), 1);

    assert_eq!(tree.remove("11"), None);
    assert_eq!(tree.get("11"), None);
    assert_eq!(tree.len(), 1);

    assert_eq!(tree.remove("12"), Some(("12".to_owned(), 12)));
    assert_eq!(tree.len(), 0);

    assert_eq!(tree.remove("12"), None);
    assert_eq!(tree.get("12"), None);
    assert_eq!(tree.len(), 0);
    assert!(tree.is_empty());
}

#[test]
fn clear() {
    let mut tree = RBTree::new();
    tree.insert(1, 'a');
    tree.insert(2, 'b');

    tree.clear();

    assert!(tree.is_empty());
    assert_eq!(tree.len(), 0);
}

#[test]
fn mutate() {
    let mut tree = RBTree::new();
    tree.insert("11".to_string(), 11);

    *tree.get_mut("11").expect("key - 11") += 1;
    assert_eq!(tree.get("11"), Some(&12));

    *tree.get_key_val_mut("11").expect("key - 11").1 += 2;
    assert_eq!(tree.get_key_val("11"), Some((&"11".to_string(), &14)));
}

#[test]
fn test_many() {
    let mut tree = RBTree::new();
    let r1 = 0..400;
    let r2 = r1.clone().rev().filter(|i| i % 4 != 0);
    let r3 = 400..1200;
    let r4 = r3.clone().rev().filter(|i| i % 12 != 0);

    for i in r1.clone() {
        assert_eq!(tree.insert(1000 + i, i), None);
    }
    for i in r1.clone() {
        assert_eq!(tree.get(&(1000 + i)), Some(&i));
    }
    for i in r2.clone() {
        assert_eq!(tree.remove(&(1000 + i)), Some((1000 + i, i)));
    }
    for i in r2.clone() {
        assert_eq!(tree.get(&(1000 + i)), None);
    }
    for i in r3.clone() {
        assert_eq!(tree.insert(1000 + i, i), None);
    }
    for i in r3.clone() {
        assert_eq!(tree.get(&(1000 + i)), Some(&i));
    }
    for i in r4.clone() {
        assert_eq!(tree.remove(&(1000 + i)), Some((1000 + i, i)));
    }
    for i in r4.clone() {
        assert_eq!(tree.get(&(1000 + i)), None);
    }
}

#[test]
fn from_iterator() {
    let tree: RBTree<i32, char> = [(1, 'a'), (2, 'b'), (3, 'c')].into_iter().collect();

    assert_eq!(tree.len(), 3);
    assert_eq!(tree.get(&1), Some(&'a'));
    assert_eq!(tree.get(&2), Some(&'b'));
    assert_eq!(tree.get(&3), Some(&'c'));
}

#[test]
fn from_array() {
    let tree: RBTree<_, _> = [(1, 'a'), (2, 'b'), (3, 'c')].into();

    assert_eq!(tree.len(), 3);
    assert_eq!(tree.get(&1), Some(&'a'));
    assert_eq!(tree.get(&2), Some(&'b'));
    assert_eq!(tree.get(&3), Some(&'c'));
}

#[test]
fn into_vec() {
    let tree: RBTree<_, _> = [(1, 'a'), (2, 'b'), (3, 'c')].into();

    let vec: Vec<_> = tree.into();
    assert_eq!(vec, [(1, 'a'), (2, 'b'), (3, 'c')]);
}

#[test]
fn into_iter() {
    let tree: RBTree<_, _> = [(2, 'b'), (3, 'c'), (1, 'a')].into();

    let mut items = vec![];
    for item in tree {
        items.push(item);
    }
    assert_eq!(items, [(1, 'a'), (2, 'b'), (3, 'c')]);
}

#[test]
fn into_iter_ref() {
    let tree: RBTree<_, _> = [(2, 'b'), (3, 'c'), (1, 'a')].into();

    let mut items = vec![];
    for item in &tree {
        items.push(item);
    }
    assert_eq!(items, [(&1, &'a'), (&2, &'b'), (&3, &'c')]);
}

#[test]
fn into_iter_mut() {
    let mut tree: RBTree<_, _> = [(2, 'b'), (3, 'c'), (1, 'a')].into();

    let mut items = vec![];
    for item in &mut tree {
        *item.1 = '0';
        items.push(item);
    }
    assert_eq!(items, [(&1, &mut '0'), (&2, &mut '0'), (&3, &mut '0')]);
    assert_eq!(tree.get(&1), Some(&'0'));
    assert_eq!(tree.get(&2), Some(&'0'));
    assert_eq!(tree.get(&3), Some(&'0'));
}

#[test]
fn iter() {
    let tree: RBTree<_, _> = [(2, 'b'), (3, 'c'), (1, 'a')].into();

    let mut items = vec![];
    for item in tree.iter() {
        items.push(item);
    }
    assert_eq!(items, [(&1, &'a'), (&2, &'b'), (&3, &'c')]);
}

#[test]
fn iter_mut() {
    let mut tree: RBTree<_, _> = [(2, 'b'), (3, 'c'), (1, 'a')].into();

    let mut items = vec![];
    for item in tree.iter_mut() {
        *item.1 = '0';
        items.push(item);
    }
    assert_eq!(items, [(&1, &mut '0'), (&2, &mut '0'), (&3, &mut '0')]);
    assert_eq!(tree.get(&1), Some(&'0'));
    assert_eq!(tree.get(&2), Some(&'0'));
    assert_eq!(tree.get(&3), Some(&'0'));
}

#[test]
fn drain() {
    let mut tree: RBTree<_, _> = [(2, 'b'), (3, 'c'), (1, 'a')].into();

    let items: Vec<_> = tree.drain().collect();

    assert_eq!(tree.len(), 0);
    assert!(tree.is_empty());
    assert_eq!(items, [(1, 'a'), (2, 'b'), (3, 'c')]);
}

#[test]
fn indexing() {
    let tree: RBTree<_, _> = [
        (2, "b".to_owned()),
        (3, "c".to_owned()),
        (1, "a".to_owned()),
    ]
    .into();

    assert_eq!(tree[&1], "a");
    assert_eq!(tree[&2], "b");
    assert_eq!(tree[&3], "c");
}

#[test]
fn clone() {
    let mut tree: RBTree<_, _> = [(1, 'a'), (2, 'b'), (3, 'c')].into();
    let clone = tree.clone();
    tree.clear();

    assert_eq!(clone.len(), 3);
    assert_eq!(clone[&1], 'a');
    assert_eq!(clone[&2], 'b');
    assert_eq!(clone[&3], 'c');
}
