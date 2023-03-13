#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut dp: Vec<Vec<i64>> = vec![vec![0; n + 1]; n + 1];
    dp[n][1..n + 1].clone_from_slice(&a[0..n]);
    for i in 1..n {
        let ii: usize = n - i;
        for j in 1..=ii {
            if ii % 2 == 1 {
                dp[ii][j] = dp[ii + 1][j].max(dp[ii + 1][j + 1]);
            } else {
                dp[ii][j] = dp[ii + 1][j].min(dp[ii + 1][j + 1]);
            }
        }
    }
    println!("{}", dp[1][1]);
}
