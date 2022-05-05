use proconio::marker::*;
use proconio::{fastout, input};

const MOD: i64 = 998244353;

// グローバル変数
struct Global {
  s: Chars,
}

// 関数はメソッドとして実装する
impl Global {
  // fn hoge(&mut self)
}

#[fastout()]
fn main() {
  input! {
    s: Chars,
  };
  // let mut g = Global { s };
  let sum = s.iter().fold(0, |sum, a| sum + (*a as i32 - 48));
  println!("{}", 45 - sum);
}
