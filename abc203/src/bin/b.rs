use proconio::marker::*;
use proconio::{fastout, input};

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
    k: usize,
  };
  // let mut g = Global { n };
  let mut ans = 0;
  for i in 1..=n {
    for j in 1..=k {
      ans += 100 * i + j;
    }
  }
  println!("{}", ans);
}
