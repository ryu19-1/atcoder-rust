use proconio::input;

fn main() {
  input! {
    n: String,
  };
  let ans = n.trim_end_matches('0');
  let rev: String = ans.chars().rev().collect();
  if ans == rev {
    println!("Yes");
  } else {
    println!("No");
  }
}
