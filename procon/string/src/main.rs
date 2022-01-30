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

    // chars 関数
    for c in hello.chars() {
        println!("{}", c);
    }

    let s_chars: &str = "打打打打打打打打";
    // chars 型の文字リテラルは、1つの文字をシングルクォートで囲う
    let da: char = '打';
    for c in s_chars.chars() {
        assert_eq!(c, da);
    }

    // bytes 関数
    let s_bytes = "有難う";
    for c in s_bytes.bytes() {
        println!("{:x}", c);
    }

    // to_string
    let ten: i32 = 10;
    assert_eq!(ten.to_string(), "10");

    let float_num: f64 = 12.0;
    assert_eq!(float_num.to_string(), "12");

    let ace: char = 'A';
    assert_eq!(ace.to_string(), "A");
}
