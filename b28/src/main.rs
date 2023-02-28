#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {n: usize}
    let mut a = vec![0; n + 1];
    a[1] = 1;
    a[2] = 1;
    for i in 2..n {
        a[i + 1] = a[i] + a[i - 1];
        a[i + 1] %= 1_000_000_007;
    }
    println!("{}", a[n]);
}
