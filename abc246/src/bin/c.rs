use proconio::input;
use std::cmp::min;

fn main() {
  input! {
    n: usize,
    mut k: isize,
    x: isize,
    mut a: [isize;n],
  };
  let mut ans: isize = a.iter().sum();
  let mut m: isize = (0..n).map(|i| a[i] / x).sum();
  m = min(m, k);
  ans -= m * x;
  k -= m;

  a = (0..n).map(|i| a[i] % x).collect();
  a.sort();
  a.reverse();
  for aa in a {
    if k == 0 {
      break;
    }
    ans -= aa;
    k -= 1;
  }

  println!("{}", ans);
}
