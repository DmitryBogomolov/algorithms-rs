pub fn array() -> Vec<i32> {
    let v = vec![3, 2, 1, 4, 1, 6, 8, 2, 5, 6, 3, 2, 8, 1, 7, 9];
    let mut r: Vec<i32> = Vec::new();
    r.extend(v.iter().map(|t| { t + 10 }));
    r.extend(v.iter().map(|t| { t + 20 }));
    r.extend(v.iter().map(|t| { t + 50 }));
    r.extend(v.iter().map(|t| { t + 60 }));
    r
}

pub fn asc<T: PartialOrd>(lhs: &T, rhs: &T) -> bool {
    lhs < rhs
}

pub fn dsc<T: PartialOrd>(lhs: &T, rhs: &T) -> bool {
    lhs > rhs
}

pub fn array_asc() -> Vec<i32> {
    let mut ar = array();
    ar.sort_by(|lhs, rhs| { lhs.cmp(rhs) });
    ar
}

pub fn array_dsc() -> Vec<i32> {
    let mut ar = array();
    ar.sort_by(|lhs, rhs| { rhs.cmp(lhs) });
    ar
}
