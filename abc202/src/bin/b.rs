use proconio::input;

fn main() {
  input! {
    s: String,
  };
  let mut rev_s = vec![];
  for e in s.chars().rev() {
    match e {
      '6' => rev_s.push('9'),
      '9' => rev_s.push('6'),
      _ => rev_s.push(e),
    }
  }
  let ans: String = rev_s.into_iter().collect();
  println!("{}", ans);
}
