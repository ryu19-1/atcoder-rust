use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    _n: usize,
    x: u128,
    s: Chars,
  };
  let mut stack = Vec::new();
  // stackの一番上がLorRと今追加するものがUなら削除
  for t in s {
    if stack.is_empty() {
      stack.push(t);
    } else if t == 'U' && *stack.last().unwrap() != 'U' {
      stack.pop();
    } else {
      stack.push(t);
    }
    // println!("{:?}", stack);
  }

  let mut ans = x;
  for ss in stack {
    match ss {
      'L' => ans *= 2,
      'R' => ans = ans * 2 + 1,
      _ => {
        if ans > 1 {
          ans /= 2
        }
      }
    }
  }
  println!("{}", ans);
}
