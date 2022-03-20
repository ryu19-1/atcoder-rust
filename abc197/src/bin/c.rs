use proconio::input;
use std::cmp::min;

fn main() {
  input! {
    n: u32,
    a: [i32;n],
  };
  let mut ans = 2i32.pow(30) - 1;
  for i in 0..(2i32.pow(n - 1)) {
    // a[:i+1] とa[i+1:]に分ける
    let mut res = 0;
    let mut tmp = a[0];
    for j in 0..((n - 1) as usize) {
      if i & (1 << j) > 0 {
        // 1: jとj+1番目は区別する
        res ^= tmp;
        tmp = a[j + 1];
      } else {
        // 0: jとj+1番目は同じグループ
        tmp |= a[j + 1];
      }
    }
    res ^= tmp;
    ans = min(ans, res);
  }
  println!("{}", ans);
}
