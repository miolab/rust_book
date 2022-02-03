fn main() {
    let vec = digits();
    assert_eq!(vec, [
        0, 1, 2, 3,
    ]);
}

fn digits() -> Vec<i32> {
    let mut v = Vec::new();
    for i in 0..4 {
        v.push(i);
    }
    v
}
