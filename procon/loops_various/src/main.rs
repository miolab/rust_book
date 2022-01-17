fn main() {
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

    for i in 21..25 {
        println!("{},", i)
    }
    for i in 31..=35 {
        println!("{},", i)
    }
}
