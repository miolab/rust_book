use proconio::input;

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
}
