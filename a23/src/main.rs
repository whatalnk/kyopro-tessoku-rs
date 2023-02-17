#![allow(clippy::many_single_char_names)]
#![allow(clippy::needless_range_loop)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[i64; n]; m],
    }
    let mut dp = vec![vec![1_000_000_000; 1024]; 109];
    dp[0][0] = 0;
    for i in 1..=m {
        for j in 0..(1 << n) {
            let mut already = vec![0; n + 9];
            for k in 1..=n {
                if (j / (1 << (k - 1))) % 2 != 0 {
                    already[k] = 1;
                }
            }
            let mut v = 0;
            for k in 1..=n {
                if already[k] == 1 || a[i - 1][k - 1] == 1 {
                    v += 1 << (k - 1);
                }
            }
            dp[i][j] = dp[i][j].min(dp[i - 1][j]);
            dp[i][v] = dp[i][v].min(dp[i - 1][j] + 1);
        }
    }
    if dp[m][(1 << n) - 1] == 1_000_000_000 {
        println!("-1");
    } else {
        println!("{}", dp[m][(1 << n) - 1]);
    }
}
