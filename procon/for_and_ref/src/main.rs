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

    let elem = (10, 20.0);
    let (ref num, ref wt) = elem;
    assert_eq!(*num, 10);
    assert_eq!(*wt, 20.0);

    // ref を使った for loop 式
    for (ref number, ref weight) in &elements {
        println!("{}: {:.1}", number, weight)
    }
    // 上記式の ref は省略可能
    for (number, weight) in &elements {
        println!("{}: {:.1}", number, weight)
    }
}
