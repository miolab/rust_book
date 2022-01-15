use proconio::input;

fn main() {
    input! {
        arr: [i32; 4],
    }

    let mut answer = 100;
    for &i in &arr {
        if i < answer {
            answer =i;
        }
    }
    println!("{}", answer);
}
