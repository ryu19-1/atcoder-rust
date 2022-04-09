use proconio::input;
use std::collections::HashMap;

fn main() {
  input! {
    n: usize,
    a: [usize;n],
  };
  let mut count: HashMap<usize, usize> = HashMap::new();
  // println!("{:?}", a);
  for item in a {
    let c = count.entry(item).or_insert(0);
    *c += 1;
  }
  for i in 0..=2000 {
    // println!("{}", i);
    if count.get(&i) == None {
      println!("{}", i);
      break;
    }
  }
}
