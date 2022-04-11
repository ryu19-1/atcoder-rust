use proconio::marker::*;
use proconio::{fastout, input};
use std::collections::HashMap;

const MOD: usize = 998244353;

// グローバル変数
#[derive(Clone)]
struct Global {
  f: HashMap<usize, usize>,
  g: HashMap<usize, usize>,
}

impl Global {
  fn calc(&mut self, n: usize) {
    for i in 1..=n {
      if i == 1 {
        self.f.insert(1, 2);
      } else if i == 2 {
        self.f.insert(2, 3);
      } else {
        self.f.insert(
          i,
          (self.f.get(&(i - 1)).unwrap() + self.f.get(&(i - 2)).unwrap()) % MOD,
        );
      }
    }
    for i in 1..=n {
      if i == 1 {
        self.g.insert(1, 1);
      } else if i == 2 {
        self.g.insert(2, 3);
      } else if i == 3 {
        self.g.insert(3, 4);
      } else {
        self.g.insert(
          i,
          (self.f.get(&(i - 1)).unwrap() + self.f.get(&(i - 3)).unwrap()) % MOD,
        );
      }
    }
  }
}

pub struct Dsu {
  n: usize,
  // root node: -1 * component size
  // otherwise: parent
  parent_or_size: Vec<i32>,
}

impl Dsu {
  pub fn new(size: usize) -> Self {
    Self {
      n: size,
      parent_or_size: vec![-1; size],
    }
  }

  pub fn merge(&mut self, a: usize, b: usize) -> usize {
    assert!(a < self.n);
    assert!(b < self.n);
    let (mut x, mut y) = (self.leader(a), self.leader(b));
    if x == y {
      return x;
    }
    if -self.parent_or_size[x] < -self.parent_or_size[y] {
      std::mem::swap(&mut x, &mut y);
    }
    self.parent_or_size[x] += self.parent_or_size[y];
    self.parent_or_size[y] = x as i32;
    x
  }

  pub fn same(&mut self, a: usize, b: usize) -> bool {
    assert!(a < self.n);
    assert!(b < self.n);
    self.leader(a) == self.leader(b)
  }

  pub fn leader(&mut self, a: usize) -> usize {
    assert!(a < self.n);
    if self.parent_or_size[a] < 0 {
      return a;
    }
    self.parent_or_size[a] = self.leader(self.parent_or_size[a] as usize) as i32;
    self.parent_or_size[a] as usize
  }

  pub fn size(&mut self, a: usize) -> usize {
    assert!(a < self.n);
    let x = self.leader(a);
    -self.parent_or_size[x] as usize
  }

  pub fn groups(&mut self) -> Vec<Vec<usize>> {
    let mut leader_buf = vec![0; self.n];
    let mut group_size = vec![0; self.n];
    for i in 0..self.n {
      leader_buf[i] = self.leader(i);
      group_size[leader_buf[i]] += 1;
    }
    let mut result = vec![Vec::new(); self.n];
    for i in 0..self.n {
      result[i].reserve(group_size[i]);
    }
    for i in 0..self.n {
      result[leader_buf[i]].push(i);
    }
    result
      .into_iter()
      .filter(|x| !x.is_empty())
      .collect::<Vec<Vec<usize>>>()
  }
}

#[fastout()]
fn main() {
  input! {
    n: usize,
    p: [Usize1;n],
    q: [Usize1;n],
  };
  let mut dsu = Dsu::new(n);
  for i in 0..n {
    dsu.merge(p[i], q[i]);
  }
  let f = HashMap::new();
  let g = HashMap::new();
  let mut global = Global { f, g };
  global.calc(n);
  let mut ans = 1;
  for item in dsu.groups().iter() {
    ans = ans * global.g.get(&item.len()).unwrap() % MOD;
  }
  println!("{}", ans);
}
