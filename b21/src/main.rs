#![allow(clippy::many_single_char_names)]
#![allow(clippy::needless_range_loop)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut dp = vec![vec![0; 1009]; 1009];
    for i in 0..n {
        dp[i][i] = 1;
    }
    for i in 0..(n - 1) {
        if s[i] == s[i + 1] {
            dp[i][i + 1] = 2;
        } else {
            dp[i][i + 1] = 1;
        }
    }
    for length in 2..n {
        for l in 0..(n - length) {
            let r = l + length;
            if s[l] == s[r] {
                dp[l][r] = dp[l][r - 1].max(dp[l + 1][r].max(dp[l + 1][r - 1] + 2));
            } else {
                dp[l][r] = dp[l][r - 1].max(dp[l + 1][r]);
            }
        }
    }
    println!("{}", dp[0][n - 1]);
}
