use proconio::input;
use proconio::marker::Chars;
use std::cmp::*;
use std::collections::HashMap;

fn main() {
  input! {
    n: usize,
    xy: [(i32,i32);n],
    s: Chars,
  };
  // 一番左のR < 一番右のLなら衝突する
  let mut hashmap: HashMap<i32, (i32, i32)> = HashMap::new();
  const INF: i32 = 1e12 as i32;
  for i in 0..n {
    if s[i] == 'L' {
      match hashmap.get_mut(&xy[i].1) {
        Some(value) => *value = (max(value.0, xy[i].0), value.1),
        None => {
          hashmap.insert(xy[i].1, (xy[i].0, INF));
        }
      }
    } else {
      match hashmap.get_mut(&xy[i].1) {
        Some(value) => *value = (value.0, min(value.1, xy[i].0)),
        None => {
          hashmap.insert(xy[i].1, (-1, xy[i].0));
        }
      }
    }
  }
  for (_, value) in hashmap.iter_mut() {
    // print!("{:?}", value);
    if value.0 < INF && value.0 > value.1 && value.1 > -1 {
      println!("Yes");
      return;
    }
  }
  println!("No");
}
