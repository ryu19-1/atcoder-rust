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
    s: String,
  };
  // let mut g = Global {n};
  match s.len() {
    1 => println!("{}{}{}{}{}{}", s, s, s, s, s, s),
    2 => println!("{}{}{}", s, s, s),
    3 => println!("{}{}", s, s),
    _ => unreachable!(),
  }
}
