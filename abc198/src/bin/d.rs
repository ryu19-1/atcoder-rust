use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::{HashMap, HashSet};

fn main() {
  input! {
    s: [Chars;3],
  };
  let mut set = HashSet::new();
  for i in 0..3 {
    for &t in &s[i] {
      set.insert(t);
    }
  }
  if set.len() > 10 {
    return println!("UNSOLVABLE");
  }
  let l = set.len();
  let list: Vec<char> = set.into_iter().collect();
  // println!("{:?}", list);
  for perm in (0..=9).permutations(l) {
    let mut map = HashMap::new();
    // let mut map = vec![0; 26];
    for (idx, v) in list.iter().enumerate() {
      map.insert(*v, perm[idx].to_string());
    }
    let mut t = vec![vec![]; 3];
    for i in 0..3 {
      for v in s[i].iter().rev() {
        t[i].push(map.get(v).unwrap());
      }
    }
    // println!("{:?}: {:?}", s, t);
    // t[0].iter().map(|x| x.to_string()).collect()
    let r1: i64 = t[0]
      .iter()
      .map(ToString::to_string)
      .collect::<String>()
      .parse()
      .unwrap();
    let r2: i64 = t[1]
      .iter()
      .map(ToString::to_string)
      .collect::<String>()
      .parse()
      .unwrap();
    let r3: i64 = t[2]
      .iter()
      .map(ToString::to_string)
      .collect::<String>()
      .parse()
      .unwrap();
    if r1 > 0 && r2 > 0 && r1 + r2 == r3 {
      for i in 0..3 {
        println!(
          "{}",
          t[i].iter().map(ToString::to_string).collect::<String>()
        );
      }
      return;
    }
  }
  println!("UNSOLVABLE")
}
