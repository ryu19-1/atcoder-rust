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
    m: usize,
    mut a: [usize;n],
    mut b: [usize;m],
  };
  let mut used = vec![false; n];
  for bb in b {
    let mut flag = false;
    for i in 0..n {
      if a[i] == bb && !used[i] {
        flag = true;
        used[i] = true;
        break;
      }
    }
    if !flag {
      println!("No");
      return;
    }
  }
  println!("Yes");
}
