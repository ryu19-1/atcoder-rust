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
    a: [usize;10],
  };
  // let mut g = Global { n };
  let mut now = 0;
  for _ in 0..3 {
    now = a[now];
  }
  println!("{}", now);
}
