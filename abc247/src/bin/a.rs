use proconio::marker::*;
use proconio::{fastout, input};

const MOD: i64 = 998244353;

#[fastout()]
fn main() {
  input! {
    s: String,
  };
  println!("0{}", &s[..3]);
}
