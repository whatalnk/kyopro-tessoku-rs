#![allow(clippy::many_single_char_names)]
#![allow(clippy::needless_range_loop)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        pa: [(usize, i64); n],
    }
    let mut p = vec![0; n + 2];
    let mut a = vec![0; n + 2];
    for i in 0..n {
        p[i + 1] = pa[i].0;
        a[i + 1] = pa[i].1;
    }
    let mut dp = vec![vec![0; 2009]; 2009];
    dp[1][n] = 0;
    for i in 0..=(n - 2) {
        let length = n - 2 - i;
        for l in 1..=(n - length) {
            let r = l + length;
            let score1 = if l <= p[l - 1] && p[l - 1] <= r {
                a[l - 1]
            } else {
                0
            };
            let score2 = if l <= p[r + 1] && p[r + 1] <= r {
                a[r + 1]
            } else {
                0
            };
            if l == 1 {
                dp[l][r] = dp[l][r + 1] + score2;
            } else if r == n {
                dp[l][r] = dp[l - 1][r] + score1;
            } else {
                dp[l][r] = (dp[l - 1][r] + score1).max(dp[l][r + 1] + score2);
            }
        }
    }
    let mut ans = 0;
    for i in 1..=n {
        ans = ans.max(dp[i][i]);
    }
    println!("{}", ans);
}
