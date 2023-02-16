#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n-1],
        bb: [usize; n-2],
    }
    let mut a = vec![0; n + 9];
    a[2..(n + 1)].clone_from_slice(&aa[..(n - 1)]);
    let mut b = vec![0; n + 9];
    b[3..(n + 1)].clone_from_slice(&bb[..(n - 2)]);
    let mut dp = vec![1_000_000_000; n + 9];
    dp[1] = 0;
    for i in 1..n {
        dp[i + 1] = dp[i + 1].min(dp[i] + a[i + 1]);
        dp[i + 2] = dp[i + 2].min(dp[i] + b[i + 2]);
    }
    println!("{}", dp[n]);
}
