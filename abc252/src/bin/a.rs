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
  };
  // let mut g = Global {n};
  let ans = "abcdefghijklmnopqrstuvwxyz";
  println!("{}", ans.chars().nth(n - 97).unwrap());
}
