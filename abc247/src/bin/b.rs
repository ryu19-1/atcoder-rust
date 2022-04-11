use proconio::marker::*;
use proconio::{fastout, input};

const MOD: i64 = 998244353;

#[fastout()]
fn main() {
  input! {
    n: usize,
    st: [(String, String);n],
  };
  for i in 0..n {
    let mut a = false;
    let mut b = false;
    for j in 0..n {
      if i == j {
        continue;
      }
      if st[i].0 == st[j].0 || st[i].0 == st[j].1 {
        a = true;
      }
      if st[i].1 == st[j].1 || st[i].1 == st[j].0 {
        b = true;
      }
      if a && b {
        println!("No");
        return;
      }
    }
  }
  println!("Yes");
}
