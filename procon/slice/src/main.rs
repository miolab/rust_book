fn main() {
    // スライス型 [T]
    // - 配列 [T; N] の ; N を取り去った形
    // - [T] 型の変数を宣言することはできない
    let mut ref_slice: &[i32];

    let array = [1, 2, 3];
    ref_slice = &array;
    println!("{:?}", ref_slice);

    let vector = vec![4, 5, 6];
    ref_slice = &vector;
    println!("{:?}", ref_slice);
}
