use proconio::input;

fn main() {
  input! {
    a: f64,
    b: f64,
  };
  let dist = (a.powf(2f64) + b.powf(2f64)).powf(0.5);
  println!("{} {}", a / dist, b / dist);
}
