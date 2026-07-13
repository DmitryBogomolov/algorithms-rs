pub struct Tester {
    pub tag: char,
    pub value: i32,
}

impl std::fmt::Debug for Tester {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.value, self.tag)
    }
}

pub fn empty() -> Vec<Tester> {
    vec![]
}

pub fn array() -> Vec<Tester> {
    vec![
        Tester{tag: 'a', value: 3},
        Tester{tag: 'a', value: 2},
        Tester{tag: 'a', value: 1},
        Tester{tag: 'a', value: 4},
        Tester{tag: 'b', value: 1},
        Tester{tag: 'a', value: 6},
        Tester{tag: 'a', value: 8},
        Tester{tag: 'b', value: 2},
        Tester{tag: 'a', value: 5},
        Tester{tag: 'b', value: 6},
        Tester{tag: 'b', value: 3},
        Tester{tag: 'c', value: 2},
        Tester{tag: 'b', value: 8},
        Tester{tag: 'a', value: 9},
        Tester{tag: 'a', value: 7},
        Tester{tag: 'c', value: 1},
    ]
}

pub fn asc(lhs: &Tester, rhs: &Tester) -> bool {
    lhs.value < rhs.value
}

pub fn dsc(lhs: &Tester, rhs: &Tester) -> bool {
    lhs.value > rhs.value
}

pub fn is_sorted(target: &[Tester], mut is_ord: impl FnMut(&Tester, &Tester) -> bool) -> bool {
    for i in 1..target.len(){
        if is_ord(&target[i], &target[i - 1]) {
            return false;
        }
        if target[i].value == target[i - 1].value && target[i].tag < target[i - 1].tag {
            return false;
        }
    }
    true
}

pub fn check_sorted(target: &[Tester], is_ord: impl FnMut(&Tester, &Tester) -> bool) {
    assert!(is_sorted(target, is_ord), "slice is not sorted: {:?}", target);
}
