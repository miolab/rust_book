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

    // for 式 (二重ループ。ラベルなし)
    for i in 0..6 {
        for j in 0..i {
            if i + j == 4 {
                break;
            }
            print!("({0}, {1}) ", i ,j)
        }
        println!();
    }

    // for 式 (二重ループ。ラベルあり)
    'outer_loop: for i in 0..6 {
        for j in 0..i {
            if i + j == 4 {
                break 'outer_loop;
            }
            print!("({0}, {1}) ", i ,j)
        }
        println!();
    }

    println!();

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
    println!("loop: (x 10)");
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
    println!("loop: (square)");
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
