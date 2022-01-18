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

    // 部分スライス
    let arr_for_partial_slice = [
        0,
        10,
        20,
        30,
        40,
        50,
    ];
    let ref_partial_slice = &arr_for_partial_slice[1..4];
    assert_eq!(ref_partial_slice[0], 10);
    assert_eq!(ref_partial_slice[1], 20);
    assert_eq!(ref_partial_slice[2], 30);

    let ref_partial_over_twenty = &arr_for_partial_slice[3..];
    assert_eq!(ref_partial_over_twenty[0], 30);
    assert_eq!(ref_partial_over_twenty[1], 40);
    assert_eq!(ref_partial_over_twenty[2], 50);

    // スライスは for で走査できる
    for i in &arr_for_partial_slice[..3] {
        println!("{}", *i);
    }

    // swap 関数
    let mut array_for_swap = [
        0,
        10,
        20,
        30,
        40,
    ];
    let ref_for_swap = &mut array_for_swap[..];
    ref_for_swap.swap(0, 3);
    println!("swap:\n{:?}", array_for_swap);

    // reverse 関数
    let mut array_for_reverse = [
        0,
        10,
        20,
        30,
        40,
    ];
    array_for_reverse.reverse();
    println!("reverse (full):\n{:?}", array_for_reverse);

    array_for_reverse[1..4].reverse();
    println!("reverse (partial):\n{:?}", array_for_reverse);

    // sort 関数
    array_for_reverse.sort();
    println!("sort:\n{:?}", array_for_reverse);
}
