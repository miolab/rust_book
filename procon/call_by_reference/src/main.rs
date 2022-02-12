use proconio::input;

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
    );

    // 可変参照を渡す
    let mut hoge = 10;
    double(&mut hoge);
    assert_eq!(hoge, 20);

    let mut hoge = 10;
    double(&mut hoge);
    assert_eq!(hoge, 20);

    // `std::mem::swap(&mut a, &mut )` 2つの可変参照を受け取って、その中身を入れ替える
    let mut x = 10;
    let mut y = 20;
    std::mem::swap(&mut x, &mut y);
    assert_eq!(x, 20);
    assert_eq!(y, 10);

    input! {
        i: usize,
        j: usize,
    }
    let mut array = [1, 2, 3, 4, 5];
    // 配列やベクタの2つの要素を入れ替えるときは、swap 関数を使う
    // `std::mem::swap(&mut array[i], &mut array[j])` は不可
    array.swap(i, j);
    println!("{:?}", array);

    print_with_debug();
}

// fn sum(v: Vec<i32>) -> i32 {... だと値渡しになってエラー
fn sum(v: &Vec<i32>) -> i32 {
    let mut ret = 0;
    // for &i in &v { // 値渡しになりエラー
    for &i in v {
        ret += i;
    }
    ret
}

// 可変参照をとる関数
fn double(x: &mut i32) {
    *x *= 2;
}

fn print_with_debug() {
    let mut x = 0;
    for i in 18..=20 {
        x += i;
        // dbg! ... 実行結果を一時的に出力する
        dbg!(x);
    }
    println!("{}", x);
}
