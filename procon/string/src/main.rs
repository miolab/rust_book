fn main() {
    // 文字列の型
    // String (Vec<T> に相当)
    let s = String::new();
    // str (スライス [T] に相当)
    let slice: &str = &s;
    println!("{}", slice); // カラ出力

    // 文字列出力
    let hello = "Hello";
    let world = "world".to_string();
    // - 文字列リテラルを使う必要がある。println!(hello); とかはエラーになる。
    println!("{}, {}!", hello, world);
}
