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
    mut abc: [i32;3],
  };
  // let mut g = Global {n};
  let b = abc[1];
  abc.sort();
  if abc[1] == b {
    println!("Yes");
  } else {
    println!("No");
  }
}
