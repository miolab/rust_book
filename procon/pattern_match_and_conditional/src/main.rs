fn main() {
    // 論駁不可能なパターン
    // let ref_slice: &[i32] = &[10, 15, 20];

    // 論駁可能なパターン
    let ref_slice: &[i32] = &[10, 15];

    if let [x, y, z] = *ref_slice {
        println!("{0}, {1}, {2}", x, y, z);
    } else {
        println!("Error! pattern match failed.");
    }
}
