#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: i64,
        wv: [(i64, i64); n],
    }
    let mut dp = vec![vec![1_000_000_000_000_000; 100_009]; 109];
    dp[0][0] = 0;
    for i in 1..=n {
        for j in 0..100_009 {
            let (ww, vv) = wv[i - 1];
            if j < vv as usize {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j].min(dp[i - 1][j - vv as usize] + ww);
            }
        }
    }
    let mut ans = 0;
    for i in 0..100_009 {
        if dp[n][i] <= w {
            ans = i;
        }
    }
    println!("{}", ans);
}
