use proconio::marker::*;
use proconio::{fastout, input};

fn f(t: usize, k: usize, s: &[char]) -> char {
  if t == 0 {
    s[k as usize]
  } else if k == 0 {
    go(s[0], t)
  } else {
    go(f(t - 1, k / 2, s), 1 + k % 2)
  }
}

// sからx進んだ文字を返す
fn go(s: char, x: usize) -> char {
  let tmp = vec!['A', 'B', 'C'];
  let idx = s as usize - 'A' as usize;
  tmp[(idx + x as usize) % 3]
}

#[fastout()]
fn main() {
  input! {
    s: Chars,
    q: usize,
    tks: [(usize,Usize1);q],
  };
  for &(t, k) in tks.iter() {
    println!("{}", f(t, k, &s));
  }
}
