use proconio::marker::*;
use proconio::{fastout, input};
use std::cmp::max;

const MOD: i64 = 998244353;

// グローバル変数
struct Global {
  n: usize,
  x: usize,
  y: usize,
  a: Vec<usize>,
}

impl Global {
  fn ok(&self, value: &usize) -> bool {
    self.y <= *value && *value <= self.x
  }

  fn split(&self) -> Vec<Vec<usize>> {
    let mut res = vec![];
    let mut tmp = vec![];
    for aa in self.a.iter() {
      if self.ok(aa) {
        tmp.push(*aa);
      } else if !tmp.is_empty() {
        res.push(tmp);
        tmp = vec![];
      }
    }
    if !tmp.is_empty() {
      res.push(tmp);
    }
    res
  }

  fn shakutori(&self, split_a: &[usize]) -> usize {
    let mut ans = 0;
    let mut r = 0;
    let mut count_x = 0;
    let mut count_y = 0;

    let m = split_a.len();
    for l in 0..m {
      while r < m && (count_x == 0 || count_y == 0) {
        if split_a[r] == self.x {
          count_x += 1;
        }
        if split_a[r] == self.y {
          count_y += 1;
        }
        r += 1;
      }
      if count_x > 0 && count_y > 0 {
        ans += m - r + 1;
      }
      // println!("{} {} {}", l, r, ans);
      if split_a[l] == self.x {
        count_x -= 1;
      }
      if split_a[l] == self.y {
        count_y -= 1;
      }
    }
    ans
  }
}

#[fastout()]
fn main() {
  input! {
    n: usize,
    x: usize,
    y: usize,
    a: [usize;n],
  };
  let g = Global { n, x, y, a };
  let mut ans = 0;
  let split_as = g.split();
  for split_a in split_as.iter() {
    ans += g.shakutori(split_a);
  }
  println!("{}", ans);
}
