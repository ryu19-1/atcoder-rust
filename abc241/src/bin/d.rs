use proconio::marker::*;
use proconio::{fastout, input};
use std::collections::BTreeSet;

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
    q: usize,
  };
  let mut a = BTreeSet::new();
  for j in 0..q {
    input! {
      i: usize,
      x: usize,
    }
    match i {
      1 => {
        a.insert((x, j));
      }
      2 => {
        input! {k: usize}
        if let Some((v, _)) = a.range(..(x, j)).rev().nth(k - 1) {
          println!("{}", v);
        } else {
          println!("-1");
        }
      }
      3 => {
        input! {k: usize}
        if let Some((v, _)) = a.range((x, 0)..).nth(k - 1) {
          println!("{}", v);
        } else {
          println!("-1");
        }
      }
      _ => unreachable!(),
    }
  }
}
