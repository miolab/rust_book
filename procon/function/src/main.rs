fn main() {
    let vec = digits(0, 4);
    assert_eq!(vec, [
        0, 1, 2, 3,
    ]);

    please_push();
    please_push();
    please_push();

    // 環境
    // 関数が異なると「環境」が異なるため、変数 a を別関数内で使うことは不可
    fnc();

    // let a = 10;
    fn fnc() {
        let b = 20;
        println!("{}", b);

        // "can't capture dynamic environment in a fn item" のエラーになる
        // println!("{}", a);
    }

    assert_eq!(
        minimum_factor(2021),
        43
    );
}

fn digits(a: i32, b: i32) -> Vec<i32> {
    let mut v = Vec::new();
    for i in a..b {
        v.push(i);
    }
    v
}

// `fn please_push() -> ()` のように返り値が `()` なら、`-> ()` の部分は省略可能
fn please_push() {
    println!("絶対に押すなよ!");
}

fn fnc_inprogress() -> i32 {
    // 「この関数の中身はあとで書く」のとき使う（関数の中身がなかったらコンパイルエラーが出る）
    todo!();
}

fn minimum_factor(n: i32) -> i32 {
    for i in 2.. {
        if i * i > n {
            break;
        } else if n % i == 0 {
            return i;
        }
    }
    n
}
