fn main() {
    let vec = {
        let mut v = Vec::new();
        for i in 0..4 {
            v.push(i);
        }
        v
    };
    assert_eq!(vec, [
        0, 1, 2, 3,
    ]);
}
