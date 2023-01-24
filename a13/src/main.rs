#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }
    let mut r = vec![0; n];
    for i in 0..(n - 1) {
        if i > 0 {
            r[i] = r[i - 1];
        }
        while r[i] < n - 1 && a[r[i] + 1] - a[i] <= k {
            r[i] += 1;
        }
    }
    let mut ans: i64 = 0;
    for i in 0..(n - 1) {
        ans += (r[i] - i) as i64;
    }
    println!("{}", ans);
}
