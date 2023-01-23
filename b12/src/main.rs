#![allow(clippy::many_single_char_names)]

use proconio::input;

fn check(x: f64, n: f64) -> f64 {
    return x * x * x + x - n;
}

fn main() {
    input! {
        n: f64,
    }
    let mut l = 1f64;
    let mut r = 100f64;
    while l < r {
        let m = (l + r) / 2.0;
        let ans = check(m, n);
        if ans.abs() <= 0.001 {
            println!("{}", m);
            return;
        } else if ans > 0f64 {
            r = m;
        } else {
            l = m;
        }
    }
}
