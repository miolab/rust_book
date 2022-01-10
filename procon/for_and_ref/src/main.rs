fn main() {
    let primes = [
        10,
        11,
        12,
    ];
    for p in &primes {
        println!("{}", p);
    }

    let elements: [(i32, f64); 3] = [
        (6, 12.0),
        (7, 14.0),
        (8, 16.0),
    ];
    for &(number, weight) in &elements {
        println!("{}: {:.1}", number, weight);
    }
}
