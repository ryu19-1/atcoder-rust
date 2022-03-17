use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    h: i32,
    w: i32,
    mut y: i32,
    mut x: i32,
    s: [Chars;h],
  };
  x -= 1;
  y -= 1;
  let mut ans = 1;
  let dx = vec![1, -1, 0, 0];
  let dy = vec![0, 0, 1, -1];
  for i in 0..4 {
    let mut now = vec![y + dy[i], x + dx[i]];
    loop {
      // println!("{:?}", now);
      if now[0] < 0 || h <= now[0] {
        break;
      }
      if now[1] < 0 || w <= now[1] {
        break;
      }
      if s[(now[0] as usize)][(now[1] as usize)] == '#' {
        break;
      } else {
        ans += 1;
        now[0] += dy[i];
        now[1] += dx[i];
      }
    }
  }
  println!("{}", ans);
}
