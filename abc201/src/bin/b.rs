use proconio::input;

fn main() {
  input! {
    n: usize,
    mut st: [(String, i32); n],
  };
  st.sort_by_cached_key(|v| v.1);
  println!("{}", st[n - 2].0);
}
