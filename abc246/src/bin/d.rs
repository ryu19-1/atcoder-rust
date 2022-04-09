use proconio::input;
use std::cmp::min;

fn calc(a: i128, b: i128, n: i128) -> bool {
  a * a * a + a * a * b + a * b * b + b * b * b >= n
}

fn calc_ans(mid_a: i128, n: i128) -> i128 {
  // bの値を二分探索
  let mut l = -1;
  let mut r = mid_a + 1;
  while r - l > 1 {
    let mid_b = l + (r - l) / 2;
    if calc(mid_a as i128, mid_b as i128, n) {
      r = mid_b;
    } else {
      l = mid_b;
    }
  }
  // println!("{},{}", mid_a, r);
  if r == mid_a + 1 {
    10i128.pow(20)
  } else {
    mid_a.pow(3) + mid_a * mid_a * r + mid_a * r * r + r.pow(3)
  }
}

fn main() {
  input! {
    n: i128,
  };
  let mut ans = 10i128.pow(20);
  // aの値を全探索
  for a in 0..=1_000_000 {
    ans = min(ans, calc_ans(a, n));
  }
  println!("{}", ans);
}
