#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        a: [i64; n],
    }
    let mut l = 1;
    let mut r = n;
    while l <= r {
        let m = (l + r) / 2;
        if x < a[m - 1] {
            r = m - 1;
        } else if x == a[m - 1] {
            println!("{}", m);
            return;
        } else {
            l = m + 1;
        }
    }
    println!("-1");
}
