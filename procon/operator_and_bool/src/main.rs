use proconio::input;

fn main() {
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

    input! {
        x: i32,
    }
    is_int_five(x);
}

fn is_int_five(a: i32) {
    if a == 5 {
        println!("this is equal to 5!");
    }
    if PartialEq::eq(&a, &5) {
        println!("this is equal to 5! (Re)");
    }
}
