use proconio::input;

fn main() {
  input! {
    s: String,
  };
  let mut it: Vec<char> = s.chars().collect();
  it.sort();
  let y: String = it.into_iter().collect();
  let y: &str = y.as_str();
  println!("{}", y);
}
