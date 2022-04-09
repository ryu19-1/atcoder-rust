use proconio::input;

fn main() {
  input! {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
  };
  if a < c {
    println!("Takahashi");
  } else if a > c {
    println!("Aoki");
  } else if b <= d {
    println!("Takahashi");
  } else {
    println!("Aoki");
  }
}
