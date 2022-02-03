fn main() {
    // コピー可能性
    // [コピー可能] i32, usize, f64 など数値型、不変参照 (&T)
    let hoge: i32 = 10;
    let copied = hoge;
    println!("{}", hoge);
    println!("{}", copied);

    // [そうでないもの] ベクタ (Vec<T>)、文字列 (String)、可変参照 (&mut T)
    let vector: Vec<i32> = vec![1, 2, 3];
    // 所有権をムーブする
    let moved = vector;
    println!("{:?}", moved);
    // 所有権を失った vector は使用できなくなる (以下はエラーを返す)
    // println!("{:?}", vector);
}
