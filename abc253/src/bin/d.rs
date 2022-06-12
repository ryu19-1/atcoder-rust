use num::integer::gcd_lcm;
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
    n: i64,
    a: i64,
    b: i64,
  };
  // let mut g = Global { n };
  let mut ans = n * (n + 1) / 2;
  let sum_a = (a + (n / a) * a) * (n / a) / 2;
  let sum_b = (b + (n / b) * b) * (n / b) / 2;
  let c = gcd_lcm(a, b).1;
  let sum_c = (c + (n / c) * c) * (n / c) / 2;
  println!("{}", ans - sum_a - sum_b + sum_c);
}
