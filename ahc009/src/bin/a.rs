extern crate rand;

use proconio::input;
use proconio::marker::Chars;

struct Global {
  p: f32,
}

impl Global {
  fn score(&self, output: &Vec<char>) -> i32 {
    30
  }

  fn make_map(&self, h: Vec<Vec<char>>, v: Vec<Vec<char>>, adj: Vec<Vec<usize>>) {
    // 盤面の隣接行列を構築
    for i in 0..20 {
      for j in 0..19 {
        if h[i][j] == '0' {
          adj[i * 20 + j].push(i * 20 + j + 1);
          adj[i * 20 + j + 1].push(i * 20 + j);
        }
      }
    }

    for i in 0..19 {
      for j in 0..20 {
        if v[i][j] == '0' {
          adj[i * 20 + j].push((i + 1) * 20 + j);
          adj[(i + 1) * 20 + j].push(i * 20 + j);
        }
      }
    }
  }
}

fn make_init_outputs(n: i32) -> Vec<Vec<char>> {
  use rand::Rng;
  const DIRECT: &[u8] = b"LRUD";
  let mut rng = rand::thread_rng();

  let mut outputs = vec![];
  for i in 0..n {
    let output: String = (0..200)
      .map(|_| {
        let idx = rng.gen_range(0, DIRECT.len());
        char::from(unsafe { *DIRECT.get_unchecked(idx) })
      })
      .collect();
    let output2 = output.chars().collect::<Vec<char>>();
    outputs.push(output2);
  }
  outputs
}

fn make_random_index(n: usize) -> usize {
  use rand::Rng;
  let mut rng = rand::thread_rng();
  rng.gen_range(0, n)
}

fn main() {
  input! {
    si: i32,
    sj: i32,
    ti: i32,
    tj: i32,
    p: f32,
    h: [Chars;20],
    v: [Chars;19],
  };
  let mut adj = vec![vec![]; 400];
  let mut g = Global { p };
  g.make_map(h, v, adj);
  println!("{:?}", adj);

  // 初期値を与える(ランダム生成で100個ほど)
  let outputs = make_init_outputs(10);
  println!("{:?}", outputs);

  // それぞれの初期値を少し変えたもので初期値を計算し、良いものを残す
  const LOOP: usize = 1000;
  for _ in 0..LOOP {
    let mut next_gen = vec![];
    for output in &outputs {
      // 少し変えた値を生成する
      for i in 0..200 {
        let mut test_output = output.clone();
        test_output[i] = 'u';
        let test_score = g.score(&test_output);
        next_gen.push((test_score, test_output));
      }
    }
  }
}
