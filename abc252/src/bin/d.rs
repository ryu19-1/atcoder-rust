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
    a: [usize;n],
  };
  // let mut g = Global { n };
  let mut map = HashMap::new();
  for i in 0..n {
    let counter = map.entry(a[i]).or_insert(0);
    *counter += 1;
  }
  let mut sum: u128 = 0;
  for m in map.values() {
    sum += m;
  }
  let mut ans: u128 = sum * sum * sum;
  for m in map.values() {
    ans -= m * m * m;
    ans -= 3 * m * m * (sum - m);
  }
  println!("{}", ans / 6);
}
