use proconio::marker::*;
use proconio::{fastout, input};
use std::collections::VecDeque;

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
    m: usize,
    ab: [(Usize1,Usize1);m],
  };
  // let mut g = Global { n };
  let mut adj = vec![vec![]; n];
  for i in 0..m {
    adj[ab[i].0].push(ab[i].1);
  }
  let mut ans = 0;
  for s in 0..n {
    let mut visit = vec![0; n];
    visit[s] = 1;
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(s);
    while !queue.is_empty() {
      let now = queue.pop_front().unwrap();
      for v in &adj[now] {
        if visit[*v] == 0 {
          visit[*v] = 1;
          queue.push_back(*v);
        }
      }
    }
    ans += visit.iter().sum::<i32>();
  }
  println!("{}", ans);
}
