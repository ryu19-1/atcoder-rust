use proconio::marker::*;
use proconio::{fastout, input};

const MOD: i64 = 998244353;

// グローバル変数
struct Global {
  n: usize,
}

// 関数はメソッドとして実装する
impl Global {
  // fn hoge(&mut self)
}

#[fastout()]
fn main() {
  input! {
    n: usize,
    a: [usize;n],
    q: usize,
    queries: [(usize,usize,usize);q],
  };
  let mut g = Global { n };
  // queryをlの順番でソートする
  let mut sorted_query = vec![];
  for i in 0..q {
    sorted_query.push([queries[i].0, queries[i].1, queries[i].2, i]);
  }
  sorted_query.sort();
  let mut counts_l = vec![0; n + 1];
  let mut ans_l = vec![0; q];
  let mut now_l = 0;
  counts_l[a[now_l]] += 1;
  for query in sorted_query {
    let l = query[0] - 1;
    let x = query[2];
    let i = query[3];
    if l == 0 {
      continue;
    }
    while now_l < l - 1 {
      now_l += 1;
      counts_l[a[now_l]] += 1;
    }
    ans_l[i] = counts_l[x];
  }

  // queryをrの順番でソートする
  let mut sorted_query = vec![];
  for i in 0..q {
    sorted_query.push([queries[i].1, queries[i].0, queries[i].2, i]);
  }
  sorted_query.sort();
  let mut counts_r = vec![0; n + 1];
  let mut ans_r = vec![0; q];
  let mut now_r = 0;
  counts_r[a[now_r]] += 1;
  for query in sorted_query {
    let r = query[0] - 1;
    let x = query[2];
    let i = query[3];
    while now_r < r {
      now_r += 1;
      counts_r[a[now_r]] += 1;
    }
    ans_r[i] = counts_r[x];
  }
  // println!("{:?}", ans_l);
  // println!("{:?}", ans_r);

  for i in 0..q {
    println!("{}", ans_r[i] - ans_l[i]);
  }
}
