use proconio::input;

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

    // for式での取り扱い
    for i in &vector {
        println!("{}", i);
    }

    // 値の追加と削除
    let mut mutable_vector = Vec::new();
    assert_eq!(mutable_vector.len(), 0);

    mutable_vector.push(10_i32);
    mutable_vector.push(20);
    mutable_vector.push(30);
    assert_eq!(
        mutable_vector,
        vec![
            10,
            20,
            30,
        ]
    );

    mutable_vector.push(40);
    assert_eq!(
        mutable_vector,
        vec![
            10,
            20,
            30,
            40,
        ]
    );

    mutable_vector.pop();
    assert_eq!(
        mutable_vector,
        vec![
            10,
            20,
            30,
        ]
    );

    println!("ベクタ生成: 要素数と要素(i32)を入力...");
    input! {
        n: usize,
        input_vector: [i32; n],
    }
    println!("ベクタ生成:\n{:?}", input_vector);
}
