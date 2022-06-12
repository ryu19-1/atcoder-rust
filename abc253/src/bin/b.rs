use num::abs;
use proconio::marker::*;
use proconio::{fastout, input};

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
    h: usize,
    w: usize,
    s: [Chars;h],
  };
  // let mut g = Global { n };
  let mut token = vec![];
  for i in 0..h {
    for j in 0..w {
      if s[i][j] == 'o' {
        token.push((i, j));
      }
    }
  }
  let ans =
    abs((token[0].0 as i32) - (token[1].0 as i32)) + abs((token[0].1 as i32) - (token[1].1 as i32));
  println!("{}", ans);
}
