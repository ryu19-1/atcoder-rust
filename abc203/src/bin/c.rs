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
    mut ab: [(usize,usize);n]
  };
  ab.sort();
  // println!("{:?}", ab);
  // let mut g = Global { n };
  let mut now = 0;
  let mut money = k;
  for (a, b) in ab {
    if now + money < a {
      println!("{}", now + money);
      return;
    } else {
      money -= a - now;
      money += b;
      now = a;
    }
  }
  println!("{}", now + money);
}
