// ---------- begin input macro ----------
// reference: https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};
    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };
    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };
    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };
    ($iter:expr, bytes) => {
        read_value!($iter, String).bytes().collect::<Vec<u8>>()
    };
    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };
    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}
// ---------- end input macro ----------

// use proconio::input;
use std::collections::VecDeque;
// use std::io::Write;

fn main() {
  input! {
    w: usize,
    h: usize,
    n: usize,
    sx: usize,
    sy: usize,
    gx: usize,
    gy: usize,
    p: [(usize, usize, usize);n],
  };

  let mut s = vec![0; w * h];

  for i in 0..n {
    let mut x = p[i].0;
    let mut y = p[i].1;
    x -= 1;
    y -= 1;
    let k = p[i].2;
    for j in 0..=k {
      let u = y * w + x;
      if j == 0 {
        s[u - k] += 1;
        if x + k + 1 < w {
          s[u + k + 1] -= 1;
        }
      } else {
        s[u + w * j - k + j] += 1;
        s[u - w * j - k + j] += 1;
        if x + k - j + 1 < w {
          s[u + w * j + k - j + 1] -= 1;
          s[u - w * j + k - j + 1] -= 1;
        }
      }
    }
  }

  for i in 0..h {
    for j in 1..w {
      s[i * w + j] += s[i * w + j - 1];
    }
  }

  let dy: Vec<i32> = vec![1, -1, 0, 0];
  let dx: Vec<i32> = vec![0, 0, 1, -1];
  let start = (sy - 1) * w + (sx - 1);
  let goal = (gy - 1) * w + (gx - 1);
  let mut queue: VecDeque<usize> = VecDeque::new();
  queue.push_back(start);
  let mut visit = vec![-1; w * h];
  visit[start] = 1;

  while !queue.is_empty() {
    let now = queue.pop_front().unwrap();
    if now == goal {
      break;
    }
    for k in 0..4 {
      let nowi = now / w;
      let nowj = now % w;
      let y = (nowi as i32) + dy[k];
      let x = (nowj as i32) + dx[k];
      if 0 <= y && y < (h as i32) && 0 <= x && x < (w as i32) {
        let u = (y as usize) * w + (x as usize);
        if s[u] == 0 && visit[u] < 0 {
          queue.push_back(u);
          visit[u] = visit[now] + 1;
        }
      }
    }
  }
  if visit[goal] > 0 {
    println!("Yes");
  } else {
    println!("No");
  }
}
