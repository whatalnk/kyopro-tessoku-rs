use proconio::input;

fn main() {
    input! {
      n: usize,
      x: i64,
      a: [i64; n],
    }
    for ai in &a {
        if ai == &x {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
