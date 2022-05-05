use proconio::marker::*;
use proconio::{fastout, input};
use std::collections::VecDeque;

const INF: usize = 1_000_000_000_000;

// グローバル変数
#[derive(Clone)]
struct Global {}

impl Global {}

#[fastout()]
fn main() {
  input! {
    n: usize,
    a: [Usize1;2],
    b: [Usize1;2],
    s: [Chars;n],
  };
  let mut deque = VecDeque::new();
  let mut dist = vec![vec![vec![INF; 4]; n]; n];
  // 各マスの四方向の入りを管理
  let mut visited = vec![vec![vec![false; 4]; n]; n];
  let dy = vec![1, 1, -1, -1];
  let dx = vec![1, -1, 1, -1];
  // スタート地点と隣接する斜め4マスを入れちゃう
  for k in 0..4 {
    let y = (a[0] as i32) + dy[k];
    let x = (a[1] as i32) + dx[k];
    if 0 <= y && y < (n as i32) && 0 <= x && x < (n as i32) && s[y as usize][x as usize] == '.' {
      deque.push_back(vec![y as usize, x as usize, k]);
      dist[y as usize][x as usize][k] = 1;
    }
  }

  while !deque.is_empty() {
    let tmp = deque.pop_front().unwrap();
    // println!("{:?}", tmp);
    let i = tmp[0];
    let j = tmp[1];
    let direct = tmp[2];
    if i == b[0] && j == b[1] {
      println!("{}", dist[i][j][direct]);
      return;
    }
    if visited[i][j][direct] {
      continue;
    }
    visited[i][j][direct] = true;
    for k in 0..4 {
      let y = (i as i32) + dy[k];
      let x = (j as i32) + dx[k];
      if 0 <= y && y < (n as i32) && 0 <= x && x < (n as i32) && s[y as usize][x as usize] == '.' {
        if k == direct {
          if dist[i][j][direct] < dist[y as usize][x as usize][k] {
            dist[y as usize][x as usize][k] = dist[i][j][direct];
            deque.push_front(vec![y as usize, x as usize, k]);
          }
        } else if dist[i][j][direct] + 1 < dist[y as usize][x as usize][k] {
          dist[y as usize][x as usize][k] = dist[i][j][direct] + 1;
          deque.push_back(vec![y as usize, x as usize, k]);
        }
      }
    }
  }
  println!("-1");
}
