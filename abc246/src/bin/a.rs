use proconio::input;

fn main() {
  input! {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
  };
  let ansx;
  if x1 == x2 {
    ansx = x3;
  } else if x1 == x3 {
    ansx = x2;
  } else {
    ansx = x1;
  }
  let ansy;
  if y1 == y2 {
    ansy = y3;
  } else if y1 == y3 {
    ansy = y2;
  } else {
    ansy = y1;
  }
  println!("{} {}", ansx, ansy);
}
