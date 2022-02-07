fn main() {
    // 参照渡し
    let vector = vec![20, 80, 60, 40];
    // let s = sum(vector); // 値渡しになりエラー
    let s = sum(&vector);
    assert_eq!(s, 200);
    println!(
        "{:?} の総和は {}",
        vector,
        s
    )
}

// fn sum(v: Vec<i32>) -> i32 { // 値渡しになりエラー
fn sum(v: &Vec<i32>) -> i32 {
    let mut ret = 0;
    // for &i in &v { // 値渡しになりエラー
    for &i in v {
        ret += i;
    }
    ret
}
