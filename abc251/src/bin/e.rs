use proconio::marker::*;
use proconio::{fastout, input};
use std::cmp::min;

const MOD: i64 = 998244353;
const INF: usize = 1_000_000_000_000_000_000;

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
    a: [usize;n],
  };
  // dp[i][0]: i番目が被覆されていない最小コスト
  // dp[i][1]: i番目が被覆されている最小コスト
  let mut dp = vec![vec![INF, INF]; n + 1];
  dp[0][0] = 0;
  for i in 0..n {
    dp[i + 1][0] = dp[i][1];
    dp[i + 1][1] = min(dp[i][0], dp[i][1]) + a[i];
  }
  let mut dp2 = vec![vec![INF, INF]; n + 1];
  dp2[0][1] = a[n - 1];
  for i in 0..n {
    dp2[i + 1][0] = dp2[i][1];
    dp2[i + 1][1] = min(dp2[i][0], dp2[i][1]) + a[i];
  }
  println!("{}", min(dp[n - 1][1], min(dp2[n - 1][0], dp2[n - 1][1])));
}
