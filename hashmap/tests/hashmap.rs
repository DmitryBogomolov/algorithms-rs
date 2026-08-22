use hashmap::HashMap;

#[test]
fn empty() {
    let map: HashMap<(), ()> = HashMap::new();
    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
}

#[test]
fn insert() {
    let mut map = HashMap::new();

    assert_eq!(map.insert("11".to_string(), 11), None);
    assert!(!map.is_empty());
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("11"), Some(&11));
    assert_eq!(map.get("11_"), None);
    // assert_eq!(map.get_kv("11"), Some((&"11".to_string(), &11)));
    // assert_eq!(map.get_kv("11_"), None);
}

#[test]
fn remove() {
    let mut tree = HashMap::new();
    tree.insert("11".to_string(), 11);

    assert_eq!(tree.remove("11"), Some(("11".to_string(), 11)));
    assert!(tree.is_empty());
    assert_eq!(tree.len(), 0);
    assert_eq!(tree.remove("11"), None);
    assert_eq!(tree.get("11"), None);
}

#[test]
fn clear() {
    let mut tree = HashMap::new();
    tree.insert(1, 'a');
    tree.insert(2, 'b');

    tree.clear();

    assert!(tree.is_empty());
    assert_eq!(tree.len(), 0);
}
