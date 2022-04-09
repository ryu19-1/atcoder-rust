use proconio::input;
use std::collections::VecDeque;

fn abs(n: i32) -> i32 {
  if n >= 0 {
    n
  } else {
    -n
  }
}

fn main() {
  input! {
    n: usize,
    k: i32,
    a: [i32;n],
    b: [i32;n],
  };
  let mut adj = vec![vec![]; 2 * n];
  for i in 0..(n - 1) {
    if abs(a[i] - a[i + 1]) <= k {
      adj[i].push(i + 1);
      adj[i + 1].push(i);
    }
    if abs(b[i] - a[i + 1]) <= k {
      adj[i + n].push(i + 1);
      adj[i + 1].push(i + n);
    }
    if abs(a[i] - b[i + 1]) <= k {
      adj[i].push(i + n + 1);
      adj[i + n + 1].push(i);
    }
    if abs(b[i] - b[i + 1]) <= k {
      adj[i + n].push(i + n + 1);
      adj[i + n + 1].push(i + n);
    }
  }
  // println!("{:?}", adj);
  let mut visit = vec![0; 2 * n];
  visit[0] = 1;
  let mut queue: VecDeque<usize> = VecDeque::new();
  queue.push_back(0);
  while !queue.is_empty() {
    let now = queue.pop_front().unwrap();
    for v in &adj[now] {
      if visit[*v] == 0 {
        visit[*v] = visit[now] + 1;
        queue.push_back(*v);
      }
    }
  }
  // println!("{:?}", visit);
  if visit[n - 1] > 0 || visit[2 * n - 1] > 0 {
    println!("Yes");
    return;
  }

  visit = vec![0; 2 * n];
  visit[n] = 1;
  let mut queue: VecDeque<usize> = VecDeque::new();
  queue.push_back(n);
  while !queue.is_empty() {
    let now = queue.pop_front().unwrap();
    for v in &adj[now] {
      if visit[*v] == 0 {
        visit[*v] = visit[now] + 1;
        queue.push_back(*v);
      }
    }
  }
  // println!("{:?}", visit);
  if visit[n - 1] > 0 || visit[2 * n - 1] > 0 {
    println!("Yes");
    return;
  }
  println!("No");
}
