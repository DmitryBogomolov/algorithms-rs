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
