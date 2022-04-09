use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    a: [usize;n],
    b: [usize;n],
    c: [usize;n],
    d: [usize;n],
  };
  let mut chocolates = vec![];
  let mut boxes = vec![];
  for i in 0..n {
    chocolates.push([a[i], b[i]]);
    boxes.push([c[i], d[i]]);
  }

  chocolates.sort();
  boxes.sort();
  println!("{:?}", chocolates);
}
