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
  let mut ans = n;
  for _ in 0..k {
    if ans % 200 == 0 {
      ans /= 200;
    } else {
      ans = ans * 1000 + 200;
    }
  }
  println!("{}", ans);
}
