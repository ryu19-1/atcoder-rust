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

fn main() {
  input! {
    n: usize,
    t: [usize;n],
  };
  // let mut g = Global { n };
  const L: usize = 100_001;
  let mut dp = vec![0; L];
  dp[0] = 1;
  for i in 0..n {
    let mut next_dp = dp.clone();
    for j in 0..L {
      if dp[j] == 1 {
        next_dp[j + t[i]] = 1;
      }
    }
    dp = next_dp;
  }
  let t_sum = t.iter().sum::<usize>();
  let mut ans = 0;
  for k in 0..L {
    if dp[k] == 1 && k * 2 <= t_sum {
      ans = k;
    }
  }
  println!("{}", t_sum - ans);
}
