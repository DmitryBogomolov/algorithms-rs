use hashmap::HashMap;

#[test]
fn empty() {
    let map: HashMap<(), ()> = HashMap::new();
    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
}
