use proconio::input;
use std::cmp::{max, min};

fn main() {
  input! {
    n: usize,
    a: [i32;n],
    b: [i32;n],
  };
  let mut a_max = a[0];
  let mut b_min = b[0];
  for i in 1..n {
    a_max = max(a_max, a[i]);
    b_min = min(b_min, b[i]);
  }
  println!("{}", max(0, b_min - a_max + 1));
}
