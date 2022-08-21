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

fn main() {
  input! {
    x: usize,
    y: usize,
  };
  // let mut g = Global {n};
  if x == y {
    println!("{}", x);
  } else {
    println!("{}", 3 - x - y);
  }
}
