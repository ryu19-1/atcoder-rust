use proconio::input;
use std::f64::consts::PI;

fn main() {
  input! {
    n: f64,
    x0: f64,
    y0: f64,
    xn2: f64,
    yn2: f64,
  };
  let r: f64 = ((x0 - xn2).powf(2f64) + (y0 - yn2).powf(2f64)).powf(0.5) / 2f64;
  let alpha: f64 = ((y0 - yn2) / (x0 - xn2)).atan();
  let theta = alpha + (2f64 * PI / n);
  if x0 < xn2 {
    let y1 = (y0 + yn2) / 2f64 - r * (theta).sin();
    let x1 = (x0 + xn2) / 2f64 - r * (theta).cos();
    println!("{} {}", x1, y1);
  } else {
    let y1 = (y0 + yn2) / 2f64 + r * (theta).sin();
    let x1 = (x0 + xn2) / 2f64 + r * (theta).cos();
    println!("{} {}", x1, y1);
  }
}
