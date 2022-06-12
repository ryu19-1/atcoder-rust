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
    a: usize,
    b: usize,
    c: usize,
  };
  // let mut g = Global { n };
  if a == b {
    println!("{}", c);
  } else if b == c {
    println!("{}", a);
  } else if c == a {
    println!("{}", b);
  } else {
    println!("0");
  }
}
