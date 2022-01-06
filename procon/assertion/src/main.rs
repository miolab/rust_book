use proconio::input;

fn main() {
    input! {
        x: i32,
    }
    assert!(
        x > 0 && x <= 100,
        "(Error!)100以下の正の整数を入力して下さい。{} は処理不可",
        x
    );

    let is_fizz = if x % 3 == 0 {
        true
    } else {
        false
    };

    let is_buzz = if x % 5 == 0 {
        true
    } else {
        false
    };

    if is_fizz && is_buzz {
        println!("fizzbuzz");
    } else if is_fizz {
        println!("fizz");
    } else if is_buzz {
        println!("buzz");
    } else {
        println!("{}", x);
    };
}
