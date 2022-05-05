use itertools::Itertools;
use proconio::input;
use proconio::marker::*;

const MOD: usize = 998_244_353;

fn modpow(x: usize, y: usize) -> usize {
  if y == 0 {
    1
  } else if y % 2 == 0 {
    modpow(x * x % MOD, y / 2)
  } else {
    x * modpow(x * x % MOD, (y - 1) / 2) % MOD
  }
}

fn main() {
  input! {
    n: usize,
    l: usize,
    s: [Chars; n]
  };

  let s_sets = (0..n)
    .map(|i| {
      s[i]
        .iter()
        .map(|c| (1usize << (*c as usize - 'a' as usize)))
        .sum::<usize>()
    })
    .collect_vec();
  let mut ans: usize = 0;
  let mut modpow_calced = vec![0; 27];
  for i in 0..=26 {
    modpow_calced[i] = modpow(i, l);
  }
  for i in 1..2u32.pow(n as u32) {
    // 積集合を求める
    let mut sets = (1 << 26) - 1;
    let mut pop_count = 0;
    for j in 0..n {
      if (1 << j) & i > 0 {
        pop_count += 1;
        sets &= s_sets[j];
      }
    }

    let s_count = (0..26).filter(|&j| ((sets >> j) & 1) == 1).count() as usize;
    if pop_count % 2 == 1 {
      ans = (ans + modpow_calced[s_count]) % MOD;
    } else {
      ans = (ans + MOD - modpow_calced[s_count]) % MOD;
    }
  }
  println!("{}", ans);
}
