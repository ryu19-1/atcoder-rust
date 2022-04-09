use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars,
    q: usize,
    tk: [(i128,i128);q],
  };
  for i in 0..q {
    let mut mods = vec![];
    let mut now_t = tk[i].0;
    if now_t > 60 as i128 {
      now_t = 60 + (now_t - 60) % 3;
    }
    let mut now_k = tk[i].1 - 1;
    for j in 0..now_t {
      mods.push(now_k % 2);
      now_k /= 2;
    }
    let tmp = vec!['A', 'B', 'C'];
    let mut ans: usize = tmp.iter().position(|&r| r == s[now_k as usize]).unwrap();
    let m = mods.len();
    for j in 0..m {
      if mods[m - 1 - j] == 0 {
        ans = (ans + 1) % 3;
      } else {
        ans = (ans + 2) % 3;
      }
      now_t -= 1;
    }

    println!("{}", tmp[ans]);
  }
}
