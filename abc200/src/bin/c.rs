use proconio::marker::*;
use proconio::{fastout, input};
use std::collections::HashMap;

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
    a: [u128;n],
  };
  // let mut g = Global { n };
  let mut count: HashMap<u128, u128> = HashMap::new();
  for item in a {
    let c = count.entry(item % 200).or_insert(0);
    *c += 1;
  }
  let mut ans = 0;
  for c in count.values() {
    ans += c * (c - 1) / 2;
  }
  println!("{}", ans);
}
