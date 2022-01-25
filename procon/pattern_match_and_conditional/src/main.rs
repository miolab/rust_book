use proconio::input;

fn main() {
    // 論駁不可能なパターン
    // let ref_slice: &[i32] = &[10, 15, 20];

    // 論駁可能なパターン
    let ref_slice: &[i32] = &[10, 15];

    if let [x, y, z] = *ref_slice {
        println!("{}, {}, {}", x, y, z);
    } else {
        println!("Error! pattern match failed.");
    }

    // 複数のパターン
    let array = [
        (1, 92),
        (3, 91),
        (95, 1),
        (94, 2),
    ];
    let mut vector = Vec::new();
    for tuple in &array {
        if let (1, value) | (value, 2) = *tuple {
            vector.push(value);
        }
    }
    assert_eq!(vector, vec![92, 94]);

    // 範囲パターン
    let tpl = (1, 2);
    if let (0..=5, x) = tpl {
        assert_eq!(x, 2);
    } else {
        // tpl = (6, 2) のようなケースでコンパイルエラーを起こす
        panic!();
    }

    // ワイルドカードパターン
    let tpl_wild_card = (3, 1, 2);
    if let (1, _, _) | (_, 1, _) | (_, _, 1) = tpl_wild_card {
        println!("1が少なくとも１つ含まれています");
    }

    for _ in 0..=2 {
        println!("ワイルドカードやるぞ！");
    }

    // while let
    let array_while = [0, 0, 0, 1, 2];
    let mut ref_slice_while = &array_while[..];
    while let [0, ..] = *ref_slice_while {
        ref_slice_while = &ref_slice_while[1..];
    }
    assert_eq!(ref_slice_while, [1, 2]);
    println!("while done.");

    // リテラルパターン
    println!("タプルのベクタを入力 (3回):");
    input! {
        input_vector: [
            (i32, i32); 3
        ],
    }
    for &tuple in &input_vector {
        if let (1, value) = tuple {
            println!("{}", value);
        } else if let (2, value) = tuple {
            println!("{}", value * value);
        } else if let (0, 0) = tuple {
            println!("break");
            break;
        } else {
            println!("?");
        }
    }

    // match 式
    println!("タプルを入力 (1 が含まれるか判定):");
    input! {
        tuple_match: (i32, i32),
    }
    match tuple_match {
        // アーム(各パターン)を並べていく
        (1, 1) => println!("どちらも 1"),
        (1, _) | (_, 1) => println!("片方のみが 1"),
        _ => println!("どちらも 1 ではない"),
    }

    // マッチガード
    let tuples_match_guard = (1, 3);
    match tuples_match_guard {
        (1, x) if x % 2 == 0 => println!("{}", x),
        _ => {}
    }

    let tpls_match_guard = [
        (2, 5),
        (4, 4),
        (1, -4),
        (-3, -3),
        (18, 18),
    ];
    let mut vector_match_guard = Vec::new();
    for &tpl in &tpls_match_guard {
        match tpl {
            (x, y) if x == y => vector_match_guard.push(x),
            _ => {}
        }
    }
    assert_eq!(vector_match_guard, [4, -3, 18]);
}
