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

    let arr = [
        1,
        2,
        3,
    ];
    let mut sum_arr = 0;
    for num in &arr {
        sum_arr += num;
    }
    println!("{}", sum_arr);
}
