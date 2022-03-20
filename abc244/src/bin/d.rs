use proconio::input;

fn main() {
  input! {
    s: [char;3],
    t: [char;3],
  };
  if s[0] == t[0] && s[1] == t[1] && s[2] == t[2] {
    println!("Yes");
    return;
  }
  if s[0] == t[1] && s[1] == t[2] && s[2] == t[0] {
    println!("Yes");
    return;
  }
  if s[0] == t[2] && s[1] == t[0] && s[2] == t[1] {
    println!("Yes");
    return;
  }
  println!("No");
}
