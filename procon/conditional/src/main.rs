use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    }
    if x > 0 && y > 0 {
        println!("両方とも正の数です");
    } else if x > 0 || y > 0 {
        println!("いずれかが正の数です");
    } else {
        println!("終了");
    }
}
