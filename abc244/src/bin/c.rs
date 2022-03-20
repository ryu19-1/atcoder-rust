use std::io::{self, Write};

fn get_line() -> String {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).ok();
  s.trim().to_string()
}
fn readln<T>() -> T
where
  T: std::str::FromStr,
  <T as std::str::FromStr>::Err: std::fmt::Debug,
{
  get_line().parse().unwrap()
}

fn main() {
  let n: usize = readln();
  let mut used = vec![0; 2 * n + 1];
  println!("1");
  io::stdout().flush().unwrap();
  used[0] = 1;
  loop {
    let m: usize = readln();
    if m == 0 {
      return;
    }
    used[m - 1] = 1;
    for i in 0..(2 * n + 1) {
      if used[i] == 0 {
        println!("{}", i + 1);
        io::stdout().flush().ok();
        used[i] = 1;
        break;
      }
    }
  }
}
