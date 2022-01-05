use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    }

    let judgement = if x > 0 && y > 0 {
        "両方とも正の数です"
    } else if x > 0 || y > 0 {
        "いずれかが正の数です"
    } else {
        "終了"
    };

    println!("{}", judgement);
}
