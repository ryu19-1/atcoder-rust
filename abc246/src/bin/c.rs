use proconio::input;
use std::collections::BTreeSet;

fn main() {
  input! {
    n: usize,
    k: i32,
    x: i32,
    a: [i32;n],
  };
  let mut set = BTreeSet::new();
  for aa in a {
    set.insert(-aa);
  }
  let mut use_remain = k;
  while use_remain > 0 {
    if set.is_empty() {
      break;
    }
    let &product = set.iter().next().unwrap();
    println!("{}", product);
    set.remove(&product);
    if -product < x {
      use_remain -= 1;
    } else {
      use_remain -= -product / x;
      set.insert(-(-product % x));
    }
    println!("{:?}, {}", set, use_remain);
  }
  let mut ans = 0;
  for s in set.into_iter() {
    ans += s;
  }
  println!("{}", ans);
}
