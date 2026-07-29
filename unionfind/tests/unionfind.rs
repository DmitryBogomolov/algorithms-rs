use unionfind::UnionFind;

#[test]
fn test_empty() {
    let uf = UnionFind::new(0);
    assert_eq!(uf.size(), 0);
    assert_eq!(uf.count(), 0);
}

#[test]
fn test_many() {
    let mut uf = UnionFind::new(1000);

    assert_eq!(uf.size(), 1000);
    assert_eq!(uf.count(), 1000);

    uf.union(0, 1);
    assert_eq!(uf.count(), 999);
    assert_eq!(uf.find(0), 0);
    assert_eq!(uf.find(1), 0);
    assert_eq!(uf.find(2), 2);
    assert_eq!(uf.find(999), 999);

    uf.union(999, 500);
    assert_eq!(uf.count(), 998);
    assert_eq!(uf.find(999), 999);
    assert_eq!(uf.find(500), 999);

    uf.union(400, 500);
    assert_eq!(uf.count(), 997);
    assert_eq!(uf.find(500), 999);
    assert_eq!(uf.find(400), 999);

    uf.union(400, 1);
    assert_eq!(uf.count(), 996);
    assert_eq!(uf.find(0), 999);
    assert_eq!(uf.find(1), 999);
    assert_eq!(uf.find(2), 2);
    assert_eq!(uf.find(999), 999);
    assert_eq!(uf.find(500), 999);
    assert_eq!(uf.find(400), 999);

    uf.collapse();
    assert_eq!(uf.count(), 996);
    assert_eq!(uf.find(0), 999);
    assert_eq!(uf.find(1), 999);
    assert_eq!(uf.find(2), 2);
    assert_eq!(uf.find(999), 999);
    assert_eq!(uf.find(500), 999);
    assert_eq!(uf.find(400), 999);
}
