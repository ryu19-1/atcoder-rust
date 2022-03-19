use proconio::input;
use std::cmp::{max, min};

fn main() {
  input! {
    n: usize,
    a: [usize;n],
  };
  let mut max0 = -1 as i32;
  let mut min0 = 0 as i32;
  let mut count: i32 = 0;
  for i in 0..n {
    if a[i] == 0 {
      count += 1;
    } else {
      count -= 1;
      count = max(0, count);
    };
    max0 = max(max0, count);
    min0 = min(min0, count);
  }

  let mut max1 = -1 as i32;
  let mut min1 = 0 as i32;
  count = 0;
  for i in 0..n {
    if a[i] == 1 {
      count += 1;
    } else {
      count -= 1;
      count = max(0, count);
    };
    max1 = max(max1, count);
    min1 = min(min1, count);
  }
  println!("{}", 1 + (max0 - min0) + (max1 - min1));
}
