use proconio::input;

fn main() {
  input! {
    mut v: i32,
    a: i32,
    b: i32,
    c: i32,
  };
  loop {
    if v - a < 0 {
      println!("F");
      return;
    }
    v -= a;
    if v - b < 0 {
      println!("M");
      return;
    }
    v -= b;
    if v - c < 0 {
      println!("T");
      return;
    }
    v -= c;
  }
}
