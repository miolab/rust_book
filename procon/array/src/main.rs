fn main() {
    let arr: [i32; 3];
    arr = [
        1,
        10,
        100,
    ];

    assert_eq!(arr[0], 1_i32);
    assert_eq!(arr[1], 10_i32);
    assert_eq!(arr[2], 100_i32);
    println!("assertion OK.");
}
