use proconio::marker::*;
use proconio::{fastout, input};
use std::collections::HashSet;

const MOD: i64 = 998244353;

// グローバル変数
struct Global {
  n: usize,
}

// 関数はメソッドとして実装する
impl Global {
  // fn hoge(&mut self)
}

#[fastout()]
fn main() {
  input! {
    n: usize,
    w: usize,
    a: [usize;n]
  };
  let mut ans = HashSet::new();
  for i in 0..n {
    ans.insert(a[i]);
    for j in 0..i {
      ans.insert(a[i] + a[j]);
      for k in 0..j {
        ans.insert(a[i] + a[j] + a[k]);
      }
    }
  }
  let mut count = 0;
  for s in ans {
    if s <= w {
      count += 1
    }
  }
  println!("{}", count);
}
