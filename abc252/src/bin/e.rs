use proconio::marker::*;
use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

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
    abc: [(Usize1,Usize1,usize); m],
  };
  let mut adj = vec![vec![]; n];
  // iter()は参照が、into_iter()はあたいが渡される
  for (i, (a, b, c)) in abc.into_iter().enumerate() {
    adj[a].push((b, c, i));
    adj[b].push((a, c, i));
  }
  const inf: usize = usize::max_value();
  let mut dist = vec![inf; n];
  dist[0] = 0;
  let mut paths = vec![inf; n];
  let mut heap = BinaryHeap::new();
  // Reverseで入れることでheap.pop()時に小さい順で取得できる
  heap.push((Reverse(0), 0));
  while !heap.is_empty() {
    let (Reverse(cost), now) = heap.pop().unwrap();
    if dist[now] < cost {
      continue;
    }
    for &(v, distance, i) in adj[now].iter() {
      let tmp = dist[now] + distance;
      if dist[v] > tmp {
        dist[v] = tmp;
        heap.push((Reverse(dist[v]), v));
        paths[v] = i;
      }
    }
  }
  let ans: Vec<String> = (1..n).map(|i| (paths[i] + 1).to_string()).collect();
  println!("{}", ans.join(" "));
}
