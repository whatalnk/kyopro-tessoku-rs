use proconio::input;

fn main() {
    input! {
      n: usize,
      x: i64,
      a: [i64; n],
    }
    for i in 0..n {
        if a[i] == x {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
