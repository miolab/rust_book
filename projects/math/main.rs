fn main() {
  println!("1 + 2 = {}", 1 + 2);
  println!("1 - 2 = {}", 1 + -2);
  println!("1 * 2 = {}", 1 * 2);

  // {integer} / {integer}
  println!(
    "10 / 3 = (商) {}, (余り) {}",
    10 / 3,
    10 % 3
  );

  // {float} / {float}
  println!(
    "10.0 / 3.0 = {}", 10.0 / 3.0);
}
