use proconio::marker::*;
use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::collections::HashMap;

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
  // let mut g = Global { n };
  let mut max_set = BTreeSet::new();
  let mut min_set = BTreeSet::new();
  let mut count = HashMap::new();
  for _ in 0..q {
    input! {
      idx: usize,
    }
    match idx {
      1 => {
        input! {x: i32,}
        let c = count.entry(x).or_insert(0);
        *c += 1;
        max_set.insert(Reverse(x));
        min_set.insert(x);
      }
      2 => {
        input! {x:i32, c: usize}
        if let Some(v) = count.get_mut(&x) {
          if *v >= c {
            *v -= c;
          } else {
            *v = 0;
          }
          if *v == 0 {
            max_set.remove(&Reverse(x));
            min_set.remove(&x);
          }
        }
      }
      _ => {
        let Reverse(max) = max_set.iter().next().unwrap();
        let min = min_set.iter().next().unwrap();
        println!("{}", max - min);
      }
    }
  }
}
