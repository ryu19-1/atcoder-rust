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
    mut a: usize,
    b: usize,
    k: usize,
  };
  // let mut g = Global { n };
  let mut ans = 0;
  while a < b {
    ans += 1;
    a *= k;
  }
  println!("{}", ans);
}
