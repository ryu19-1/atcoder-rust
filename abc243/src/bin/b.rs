use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [i32;n],
    b: [i32;n],
  };
  let mut ans1 = 0;
  let mut ans2 = 0;
  for i in 0..n {
    if a[i] == b[i] {
      ans1 += 1;
    }
    for j in 0..n {
      if a[i] == b[j] {
        ans2 += 1;
      }
    }
  }
  println!("{}", ans1);
  println!("{}", ans2 - ans1);
}
