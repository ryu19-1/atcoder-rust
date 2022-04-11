use proconio::marker::*;
use proconio::{fastout, input};

const MOD: i64 = 998244353;

#[fastout()]
fn main() {
  input! {
    n: usize,
  };
  let mut s: String = String::from("1");
  for i in 2..=n {
    let a = format!("{} {} {}", s, &i.to_string(), s);
    s = a;
  }
  println!("{}", s);
}
