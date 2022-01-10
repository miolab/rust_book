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

    let [a, b, c] = arr;
    assert_eq!(a, 1);
    assert_eq!(b, 10);
    assert_eq!(c, 100);
    println!(
        "pattern match OK. {0}, {1}, {2}",
        a,
        b,
        c
    );
}
