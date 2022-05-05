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
    mut k: usize,
  };
  let mut dp = vec![0; n * m + 1];
  dp[0] = 1;
  for i in 0..n {
    let mut next_dp = vec![0; n * m + 1];
    for j in 0..(n * m + 1) {
      if dp[j] == 0 {
        continue;
      }
      for mm in 1..=m {
        next_dp[j + mm] += dp[j];
        next_dp[j + mm] %= MOD;
      }
    }
    dp = next_dp;
    // println!("{:?}", dp);
  }
  let mut ans = 0;
  for i in 0..=k {
    ans += dp[i];
    ans %= MOD;
  }
  println!("{}", ans);
}
