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
    a: [usize;n],
    b: [usize;k],
  };
  let mut dislike = vec![false; n];
  for i in 0..k {
    dislike[b[i] - 1] = true;
  }
  let max_a = a.iter().max().unwrap();
  let mut ans = "No";
  for i in 0..n {
    if dislike[i] && a[i] == *max_a {
      ans = "Yes";
      break;
    }
  }
  println!("{}", ans);
}
