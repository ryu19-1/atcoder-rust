use proconio::marker::*;
use proconio::{fastout, input};
use std::cmp::min;

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
    s: [Chars;n],
  };
  // let mut g = Global { n };
  let mut ans = 1_000_000_000;
  // 1~0をそろえることを考える
  for i in 0..10 {
    let mut checks = vec![];
    for j in 0..n {
      let mut idx = 0;
      for k in 0..10 {
        if s[j][k] == std::char::from_digit(i, 10).unwrap() {
          idx = k;
          break;
        }
      }
      checks.push(idx);
    }
    // println!("{}: {:?}", i, checks);
    let mut counter = vec![0; 10];
    let mut countchecks = vec![];
    for j in 0..n {
      countchecks.push(checks[j] + 10 * counter[checks[j]]);
      counter[checks[j]] += 1;
    }
    let res = countchecks.iter().max().unwrap();
    // println!("{:?} {}", countchecks, res);
    ans = min(ans, *res);
  }
  println!("{}", ans);
}
