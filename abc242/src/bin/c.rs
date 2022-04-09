use proconio::input;

fn main() {
  input! {
    n: i32,
  };
  const M: i32 = 998_244_353;
  let mut dp = vec![1; 9];
  for i in 0..(n - 1) {
    let mut next_dp = vec![0; 9];
    for j in 0..9 {
      if j > 0 {
        next_dp[j - 1] += dp[j];
        next_dp[j - 1] %= M;
      }
      next_dp[j] += dp[j];
      next_dp[j] %= M;
      if j < 8 {
        next_dp[j + 1] += dp[j];
        next_dp[j + 1] %= M;
      }
    }
    dp = next_dp;
  }
  let mut ans = 0;
  for d in dp {
    ans += d;
    ans %= M;
  }
  println!("{}", ans);
}
