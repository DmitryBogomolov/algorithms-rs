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

    assert_eq!(tree.insert("11".to_string(), 11), None);
    assert!(!tree.is_empty());
    assert_eq!(tree.len(), 1);
    assert_eq!(tree.get("11"), Some(&11));
    assert_eq!(tree.get("11_"), None);
}

#[test]
fn remove() {
    let mut tree = RBTree::new();
    tree.insert("11".to_string(), 11);

    assert_eq!(tree.remove("11"), Some(("11".to_string(), 11)));
    assert!(tree.is_empty());
    assert_eq!(tree.len(), 0);
    assert_eq!(tree.remove("11"), None);
    assert_eq!(tree.get("11"), None);
}

#[test]
fn mutate() {
    let mut tree = RBTree::new();
    tree.insert("11".to_string(), 11);

    *tree.get_mut("11").unwrap() += 1;

    assert_eq!(tree.get("11"), Some(&12));
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
