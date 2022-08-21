use proconio::input;
use proconio::marker::*;

const MOD: i64 = 998244353;

// グローバル変数
struct Global {
  n: usize,
}

// 関数はメソッドとして実装する
impl Global {
  // fn hoge(&mut self)
}

fn main() {
  input! {
    n: usize,
    a: [i32; n],
  };
  // let mut g = Global { n };
  let ans: i32 = a.iter().filter(|&&x| x > 10).fold(0, |ans, x| ans + x - 10);
  println!("{}", ans);
}
