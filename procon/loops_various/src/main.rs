fn main() {
    let arr = [11, 12, 13, 14, 15, 16];
    for &i in &arr {
        if i == 12 {
            continue;
        }
        if i == 14 {
            break;
        }
        println!("{}, ", i);
    }
    println!("done");
}
