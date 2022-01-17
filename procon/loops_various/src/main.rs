use proconio::input;

fn main() {
    // for 式（continue, break）
    let arr = [11, 12, 13, 14, 15, 16];
    for &i in &arr {
        if i == 12 {
            continue;
        }
        if i == 14 {
            break;
        }
        println!("{},", i);
    }
    println!("done");

    // `..` 演算子
    for i in 21..25 {
        println!("{},", i)
    }
    for i in 31..=35 {
        println!("{},", i)
    }

    // while 式
    println!("while:");
    let mut a = 30;
    let mut vec = Vec::new();
    while a > 0 {
        vec.push(a);
        a /= 2;
    }
    println!("{:?},", vec);

    // loop 式
    println!("loop:");
    loop {
        input! {
            x: i32,
        }
        if x > 0 {
            println!("x 10: {},", x * 10);
        } else {
            break;
        }
    }

    // loop 式（break で値を返す方法）
    let val = loop {
        input! {
            x: i32,
        }
        if x > 0 {
            println!("^2: {},", x * x);
        } else {
            break x;
        }
    };
    println!("break with {}", val)
}
