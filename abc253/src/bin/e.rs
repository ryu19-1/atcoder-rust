use num::abs;
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
    k: i64,
  };
  // let mut g = Global { n };
  let mut dp = vec![vec![0; m + 1]; n + 1];
  dp[1] = vec![1; m + 1];
  for i in 1..n {
    // j=1のときの値を計算、j=2以降は差分更新する
    let mut pre_calc = 0;
    for l in 1..=m {
      if abs((l as i64) - 1) >= k {
        pre_calc += dp[i][l];
        pre_calc %= MOD;
      }
    }
    dp[i + 1][1] = pre_calc;
    // println!("{:?}", pre_calc);
    for j in 2..=m {
      let val1 = k + (j as i64) - 1;
      if 1 <= val1 && val1 <= m as i64 {
        pre_calc += MOD - dp[i][val1 as usize];
        pre_calc %= MOD;
      }
      let val2 = -k + (j as i64);
      if 1 <= val2 && val2 <= m as i64 {
        pre_calc += dp[i][val2 as usize];
        pre_calc %= MOD;
      }
      dp[i + 1][j] = pre_calc;
    }
  }
  let mut ans = 0;
  for i in 1..=m {
    ans += dp[n][i];
    ans %= MOD;
  }
  println!("{}", ans);
  // println!("{:?}", dp);
}
