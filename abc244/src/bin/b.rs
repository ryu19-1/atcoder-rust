use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    n: usize,
    t: Chars,
  };
  let mut ans = (0, 0);
  let d = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];
  let mut k = 0;
  for i in 0..n {
    if t[i] == 'S' {
      ans = (ans.0 + d[k % 4].0, ans.1 + d[k % 4].1);
    } else {
      k += 1;
    }
  }
  println!("{} {}", ans.0, ans.1);
}
