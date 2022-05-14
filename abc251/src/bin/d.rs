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
    w: usize,
  };
  let mut ans = vec![];
  for i in 1..100 {
    ans.push(i);
    ans.push(i * 100);
    ans.push(i * 10000);
  }
  println!("{}", ans.len());
  let hoge: Vec<String> = ans.iter().map(|&x| x.to_string()).collect();
  println!("{}", hoge.join(" "));
  // let mut g = Global {n};
}
