#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n-1],
        b: [i64; n-2],
    }
    let mut dp = vec![10_000_001; n + 1];
    dp[1] = 0;
    dp[2] = a[0];
    for i in 3..=n {
        dp[i] = (dp[i - 1] + a[i - 2]).min(dp[i - 2] + b[i - 3]);
    }
    println!("{}", dp[n]);
}
