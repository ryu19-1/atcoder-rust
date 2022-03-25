use proconio::input;
use std::cmp::{max, min};
use std::collections::HashMap;

struct Global {
  h: usize,
  w: usize,
  a: usize,
}

impl Global {
  fn calc(&self, states: Vec<usize>, ans: usize, count_a: usize) -> usize {
    if count_a == self.a {
      return 1;
    }
    let mut count = HashMap::new();
    for s in &states {
      let c = count.entry(s).or_insert(0);
      *c += 1;
    }
    let mut c_max: i32 = -1;
    for (k, v) in count {
      if v == 2 {
        c_max = max(c_max, *k as i32);
      }
    }
    // 畳をつなげられるか判定
    let dy: Vec<i32> = vec![0, 1];
    let dx: Vec<i32> = vec![1, 0];
    let mut res = ans;
    for v in ((c_max + 1) as usize)..(self.h * self.w) {
      let i = v / self.w;
      let j = v % self.w;
      // vが既に周りと繋がっていたらだめ
      let mut v_count = 0;
      for s in &states {
        if s == &states[v] {
          v_count += 1;
        }
      }
      if v_count != 1 {
        continue;
      }
      for k in 0..2 {
        let y = (i as i32) + dy[k];
        let x = (j as i32) + dx[k];
        if 0 <= y && y < (self.h as i32) && 0 <= x && x < (self.w as i32) {
          let u = (y * (self.w as i32) + x) as usize;
          if states[v] != states[u] {
            // uが既に繋がっていたらだめ
            let mut u_count = 0;
            for s in &states {
              if s == &states[u] {
                u_count += 1;
              }
            }
            if u_count != 1 {
              continue;
            }
            // 繋げられる
            let mut new_states = states.clone();
            let min_state = min(states[v], states[u]);
            new_states[v] = min_state;
            new_states[u] = min_state;
            res += self.calc(new_states, ans, count_a + 1);
          }
        }
      }
    }
    res
  }
}

fn main() {
  input! {
    h: usize,
    w: usize,
    a: usize,
    _: usize,
  };
  let states = (0..h * w).collect::<Vec<usize>>();
  let ans = Global { h, w, a }.calc(states, 0, 0);
  println!("{}", ans);
}
