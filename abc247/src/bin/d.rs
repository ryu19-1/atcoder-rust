use proconio::marker::*;
use proconio::{fastout, input};
use std::collections::VecDeque;

const MOD: i64 = 998244353;

#[fastout()]
fn main() {
  input! {
    q: usize,
  };
  let mut deque: VecDeque<Vec<usize>> = VecDeque::new();
  for i in 0..q {
    input! {
      num: usize,
    }
    if num == 1 {
      input! {
        x: usize,
        c: usize,
      }
      deque.push_back(vec![x, c]);
    } else {
      input! {
        mut c: usize,
      }
      let mut ans = 0;
      while c > 0 {
        // 左からボールを出す
        let xc = deque.pop_front().unwrap();
        if xc[1] > c {
          ans += xc[0] * c;
          deque.push_front(vec![xc[0], xc[1] - c]);
          c = 0;
        } else {
          ans += xc[0] * xc[1];
          c -= xc[1];
        }
      }
      println!("{}", ans);
    }
  }
}
