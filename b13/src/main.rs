#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }
    let mut b = vec![0; n + 1];
    for i in 0..n {
        b[i + 1] = b[i] + a[i];
    }
    let mut r = vec![0; n + 1];
    for i in 0..n {
        if i > 0 {
            r[i] = r[i - 1];
        }
        while r[i] < n && b[r[i] + 1] - b[i] <= k {
            r[i] += 1;
        }
    }
    let mut ans: i64 = 0;
    for (i, ri) in r.iter().enumerate().take(n) {
        ans += (ri - i) as i64;
    }
    println!("{}", ans);
}
