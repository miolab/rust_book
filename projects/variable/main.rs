fn main() {
  let min_length: i32;
  let max_length: i32;

  min_length = 3;
  max_length = 5;

  println!("長辺の長さ: {}", max_length);
  println!("周長: {}", min_length * 2 + max_length * 2);
  println!("面積: {}", min_length * max_length);
}
