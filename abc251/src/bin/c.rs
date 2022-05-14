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
    st: [(String,usize);n]
  };
  // let mut g = Global { n };
  let mut ans = 0;
  let mut score = 0;
  let mut set = HashSet::new();
  for i in 0..n {
    if !set.contains(&st[i].0) && st[i].1 > score {
      ans = i;
      score = st[i].1;
    }
    set.insert(&st[i].0);
  }
  println!("{}", ans + 1);
}
