use rbtree::RBTree;

#[test]
fn empty() {
    let tree: RBTree<(), ()> = RBTree::new();
    assert!(tree.is_empty());
    assert_eq!(tree.len(), 0);
}
