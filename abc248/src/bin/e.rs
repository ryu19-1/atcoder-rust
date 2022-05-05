use num::integer::gcd;
use proconio::marker::*;
use proconio::{fastout, input};
use std::cmp::max;
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
    n: usize,
    k: usize,
    xy: [(isize,isize);n],
  };
  // let mut g = Global { n };
  let mut map: HashMap<String, usize> = HashMap::new();
  for i in 0..n {
    for j in (i + 1)..n {
      // i番目とj番目の点を結ぶ直線の傾きと切片を出す。
      let mut dx = xy[i].0 - xy[j].0;
      let mut dy = xy[i].1 - xy[j].1;
      if dx < 0 {
        dx = -dx;
        dy = -dy;
      }
      let tmp = gcd(max(dx, -dx), max(dy, -dy));
      dx /= tmp;
      dy /= tmp;
      // if dx == 0 {
      //   dy = 1;
      // } else if dy == 0 {
      //   dx = 1;
      // }
      // let d = -dx * xy[i].1 + dy * xy[i].0;
      // let tmp = gcd(gcd(max(dx, -dx), max(dy, -dy)), max(d, -d));
      let key: String = format!("{}_{}_{}", dy, -dx, dx * xy[i].1 - dy * xy[i].0);
      // println!("{} {} {}", i, j, key);
      let counter = map.entry(key).or_insert(0);
      *counter += 1;
    }
  }
  let mut ans = 0;
  println!("{:?}", map);
  for (key, value) in map.iter() {
    if *value >= k * (k - 1) / 2 {
      ans += 1;
    }
  }
  if k == 1 {
    println!("Infinity");
  } else {
    println!("{}", ans);
  }
}
