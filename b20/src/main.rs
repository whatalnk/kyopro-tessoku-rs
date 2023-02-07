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
    for i in 1..=n {
        dp[i][0] = dp[i - 1][0] + 1;
    }
    for j in 1..=m {
        dp[0][j] = dp[0][j - 1] + 1;
    }
    for i in 1..=n {
        for j in 1..=m {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i - 1][j - 1].min((dp[i - 1][j] + 1).min(dp[i][j - 1] + 1));
            } else {
                dp[i][j] = (dp[i - 1][j - 1] + 1).min((dp[i - 1][j] + 1).min(dp[i][j - 1] + 1));
            }
        }
    }
    println!("{}", dp[n][m]);
}
