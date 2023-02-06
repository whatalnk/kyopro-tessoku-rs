#![allow(clippy::many_single_char_names)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let n = s.len();
    let m = t.len();
    let mut dp = vec![vec![0; 2009]; 2009];
    for i in 0..=n {
        for j in 0..=m {
            if i >= 1 && j >= 1 && s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1].max(dp[i - 1][j - 1] + 1));
            } else if i >= 1 && j >= 1 {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            } else if i >= 1 {
                dp[i][j] = dp[i - 1][j];
            } else if j >= 1 {
                dp[i][j] = dp[i][j - 1];
            }
        }
    }
    println!("{}", dp[n][m]);
}
