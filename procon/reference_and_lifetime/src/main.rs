fn main() {
    let hoge: i8 = 100;
    let reference = &hoge;
    assert_eq!(*reference, 100_i8);
    println!("{:p}", reference);

    let fuga: i8 = 100;
    let ref reference_fuga = fuga;
    assert_eq!(*reference_fuga, 100);
    println!("{:p}", reference_fuga);
}
