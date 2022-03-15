use proconio::input;

fn main() {
  input! {
    s: String
  };
  let mut ans = 0;
  for n in 0..=9999 {
    let number: String = format!("{:04}", n);
    if check(number, &s) {
      ans += 1;
    }
  }
  println!("{}", ans);
}

fn check(number: String, s: &str) -> bool {
  let mut res = true;
  for i in 0..=9 {
    match s.chars().nth(i).unwrap() {
      'o' => {
        if is_exist(&number, i) {
          continue;
        } else {
          res = false;
          break;
        }
      }
      'x' => {
        if !is_exist(&number, i) {
          continue;
        } else {
          res = false;
          break;
        }
      }
      _ => continue,
    }
  }
  res
}

fn is_exist(number: &str, i: usize) -> bool {
  let mut res = false;
  for j in 0..=3 {
    if number.chars().nth(j).unwrap() == i.to_string().chars().next().unwrap() {
      res = true;
      break;
    }
  }
  res
}
