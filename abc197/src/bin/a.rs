use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars,
  };
  println!("{}", format!("{}{}{}", s[1], s[2], s[0]));
}
