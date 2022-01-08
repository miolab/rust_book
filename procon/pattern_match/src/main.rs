use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    let tuple: (i32, i32) = (
        a,
        b,
    );
    println!("タプルの1番目要素は、{}", tuple.0);
    println!("タプルの2番目要素は、{}", tuple.1);

    let (max, min) = if a > b {
        (a, b)
    } else {
        (b, a)
    };
    assert!(max >= min);
    println!("数値の大きさ降順にパターンマッチ: {},{}", max, min);
}
