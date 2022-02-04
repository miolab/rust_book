fn main() {
    let vec = digits();
    assert_eq!(vec, [
        0, 1, 2, 3,
    ]);

    please_push();
    please_push();
    please_push();
}

fn digits() -> Vec<i32> {
    let mut v = Vec::new();
    for i in 0..4 {
        v.push(i);
    }
    v
}

// `fn please_push() -> ()` のように返り値が `()` なら、`-> ()` の部分は省略可能
fn please_push() {
    println!("絶対に押すなよ!");
}
