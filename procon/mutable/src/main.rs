fn main() {
    let mut mutable: i32;

    mutable = 1;
    assert_eq!(mutable, 1);
    println!("{:p}", &mutable);

    mutable = 2;
    assert_eq!(mutable, 2);
    println!("{:p}", &mutable);

    mutable += 100;
    assert_eq!(mutable, 102);
}
