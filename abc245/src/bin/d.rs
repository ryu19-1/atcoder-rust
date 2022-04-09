use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    mut a: [i32;n+1],
    mut c: [i32;n+m+1],
  };
  let mut b = vec![];
  for i in 0..=m {
    let tmp_b = c[n + m - i] / a[n];
    b.push(tmp_b);
    // println!("{}", i);
    for j in 0..=n {
      // println!("{},{}", j, n + m - i - j);
      c[n + m - i - j] -= tmp_b * a[n - j];
    }
    // println!("{:?}", b);
    // println!("{:?}", c);
  }
  b.reverse();
  let ans: Vec<String> = b.iter().map(|x| x.to_string()).collect();
  println!("{}", ans.join(" "));
}
