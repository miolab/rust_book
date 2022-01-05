use proconio::input;

fn main() {
    input! {
        x: i32,
    }
    if x < 100 {
        println!("100未満です");
    }
    println!("終了");
}
