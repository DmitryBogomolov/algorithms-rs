use unionfind::UnionFind;

#[test]
fn empty() {
    let uf = UnionFind::new(0);

    assert_eq!(uf.size(), 0);
    assert_eq!(uf.count(), 0);
}

#[test]
fn one_item() {
    let uf = UnionFind::new(1);

    assert_eq!(uf.size(), 1);
    assert_eq!(uf.count(), 1);
    assert_eq!(uf.find(0), 0);
}

fn groups(uf: &UnionFind) -> Vec<usize> {
    (0..uf.size()).map(|i| uf.find(i)).collect()
}

#[test]
fn unions() {
    let mut uf = UnionFind::new(5);

    assert_eq!(uf.size(), 5);
    assert_eq!(uf.count(), 5);
    assert_eq!(groups(&uf), vec![0, 1, 2, 3, 4]);

    uf.union(0, 3);
    assert_eq!(uf.size(), 5);
    assert_eq!(uf.count(), 4);
    assert_eq!(groups(&uf), vec![0, 1, 2, 0, 4]);

    uf.union(1, 4);
    assert_eq!(uf.size(), 5);
    assert_eq!(uf.count(), 3);
    assert_eq!(groups(&uf), vec![0, 1, 2, 0, 1]);

    uf.union(2, 4);
    assert_eq!(uf.size(), 5);
    assert_eq!(uf.count(), 2);
    assert_eq!(groups(&uf), vec![0, 1, 1, 0, 1]);

    uf.union(3, 1);
    assert_eq!(uf.size(), 5);
    assert_eq!(uf.count(), 1);
    assert_eq!(groups(&uf), vec![1, 1, 1, 1, 1]);
}

#[test]
fn collapse() {
    let mut uf = UnionFind::new(5);

    uf.union(0, 1);
    uf.union(2, 3);
    uf.union(1, 4);
    uf.union(3, 0);

    uf.collapse();

    assert_eq!(uf.count(), 1);
    assert_eq!(groups(&uf), vec![0, 0, 0, 0, 0]);
}

#[test]
fn many_items() {
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

#[test]
fn clone() {
    let uf = {
        let mut uf = UnionFind::new(5);
        uf.union(0, 3);
        uf.union(1, 4);
        uf.clone()
    };

    assert_eq!(uf.size(), 5);
    assert_eq!(uf.count(), 3);
    assert_eq!(groups(&uf), vec![0, 1, 2, 0, 1]);
}
