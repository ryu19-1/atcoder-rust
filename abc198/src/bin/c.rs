use proconio::input;

fn main() {
  input! {
    r: f32,
    mut x: f32,
    mut y: f32,
  };
  let d = (x * x + y * y).powf(0.5);
  if d < r {
    println!("2");
  } else {
    println!("{}", (d / r).ceil());
  }
}
