pub fn assert_in_range(val: usize, len: usize, name: &str) {
    assert!(val < len, "{} {} out of range {}", name, val, len);
}
