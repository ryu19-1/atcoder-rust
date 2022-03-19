use num::integer::gcd;
use proconio::input;

fn main() {
  input! {
    mut l: usize,
    mut r: usize,
  };
  let mut diff = 1;
  loop {
    if gcd(l, r) == 1 {
      println!("{}", r - l);
      return;
    } else {
      for i in 0..=diff {
        if gcd(l + i, l + i + (r - l - diff)) == 1 {
          println!("{}", r - l - diff);
          return;
        }
      }
      diff += 1;
    }
  }
}
