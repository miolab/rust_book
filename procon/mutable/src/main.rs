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

    // ベクタ
    let empty_vector = Vec::<i32>::new();
    println!("空ベクタ:\n{:?}", empty_vector);

    let empty_another_vector: Vec<u32>;
    empty_another_vector = Vec::new();
    println!("空ベクタ_v2:\n{:?}", empty_another_vector);

    let mut vector: Vec<i32> = vec![
        1,
        2,
        3,
    ];
    assert_eq!(vector.len(), 3_usize);
    assert_eq!(vector[0], 1_i32);

    vector[1] = 10;
    println!("ベクタ:\n{:?}", vector);
}
