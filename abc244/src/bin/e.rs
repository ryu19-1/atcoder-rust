use proconio::input;
use std::collections::HashSet;

fn main() {
  input! {
    n: usize,
    m: usize,
    k: usize,
    mut s: usize,
    mut t: usize,
    mut x: usize,
    UV: [[usize;2];m],
  };
  s -= 1;
  t -= 1;
  x -= 1;
  let mut adj = vec![vec![]; n];
  for uv in UV {
    adj[uv[0] - 1].push(uv[1] - 1);
    adj[uv[1] - 1].push(uv[0] - 1);
  }

  let mut dp = HashSet::new();
  dp.insert(s);

  let mut visit = vec![0; 2 * n];
  visit[s] = 1;
  const M: usize = 998_244_353;

  for _ in 0..k {
    let mut next_dp = HashSet::new();
    let mut next_visit = vec![0; 2 * n];
    for v in &dp {
      for u in &adj[v % n] {
        if *u == x {
          next_dp.insert(u + n * (1 - v / n));
          next_visit[u + n * (1 - v / n)] += visit[*v];
          next_visit[u + n * (1 - v / n)] %= M;
        } else {
          next_dp.insert(u + n * (v / n));
          next_visit[u + n * (v / n)] += visit[*v];
          next_visit[u + n * (v / n)] %= M;
        }
      }
    }
    dp = next_dp;
    visit = next_visit;
  }
  println!("{}", visit[t]);
}
