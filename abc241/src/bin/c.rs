use proconio::marker::*;
use proconio::{fastout, input};

#[fastout()]
fn main() {
  input! {
    n: usize,
    s: [Chars; n],
  };
  for i in 0..n {
    for j in 0..n {
      if i + 5 < n && (0..6).filter(|&k| s[i + k][j] == '#').count() > 3
        || j + 5 < n && (0..6).filter(|&k| s[i][j + k] == '#').count() > 3
        || i + 5 < n && j + 5 < n && (0..6).filter(|&k| s[i + k][j + k] == '#').count() > 3
        || i + 5 < n && j > 5 && (0..6).filter(|&k| s[i + k][j - k] == '#').count() > 3
      {
        println!("Yes");
        return;
      }
    }
  }
  println!("No");
}
