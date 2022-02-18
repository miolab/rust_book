use proconio::input;

fn main() {
    // basic operators
    let actual_add = std::ops::Add::add(2, 3);
    assert_eq!(actual_add, 2 + 3);
    let actual_sub = std::ops::Sub::sub(20, 3);
    assert_eq!(actual_sub, 20 - 3);
    let actual_mul = std::ops::Mul::mul(20, 3);
    assert_eq!(actual_mul, 20 * 3);
    let actual_div = std::ops::Div::div(20, 3);
    assert_eq!(actual_div, 20 / 3);
    assert_eq!(actual_div, 6);
    let actual_rem = std::ops::Rem::rem(20, 3);
    assert_eq!(actual_rem, 20 % 3);
    assert_eq!(actual_rem, 2);

    // bool
    let assert_true: bool = true;
    let assert_false: bool = false;
    assert_eq!(1 == 1, assert_true);
    assert_eq!(1 == 2, assert_false);

    input! {
        x: i32,
    }
    is_int_five(x);

    for i in 0..100 {
        if is_prime(i) {
            println!("{}", i);
        }
    }
}

fn is_int_five(a: i32) {
    if a == 5 {
        println!("this is equal to 5!");
    }
    if PartialEq::eq(&a, &5) {
        println!("this is equal to 5! (Re)");
    }
}

fn is_prime(x: i32) -> bool {
    if x < 2 {
        return false;
    }
    for i in 2.. {
        if i * i > x {
            return true;
        }
        if x % i == 0 {
            return false;
        }
    }
    unreachable!();
}
