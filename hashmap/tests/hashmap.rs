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
    let mut map = HashMap::new();
    map.insert("11".to_string(), 11);

    assert_eq!(map.remove("11"), Some(("11".to_string(), 11)));
    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
    assert_eq!(map.remove("11"), None);
    assert_eq!(map.get("11"), None);
}

#[test]
fn clear() {
    let mut map = HashMap::new();
    map.insert(1, 'a');
    map.insert(2, 'b');

    map.clear();

    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
}

#[test]
fn mutate() {
    let mut map = HashMap::new();
    map.insert("11".to_string(), 11);

    *map.get_mut("11").unwrap() += 1;
    assert_eq!(map.get("11"), Some(&12));

    // *map.get_kv_mut("11").unwrap().1 += 2;
    // assert_eq!(map.get_kv("11"), Some((&"11".to_string(), &14)));
}
