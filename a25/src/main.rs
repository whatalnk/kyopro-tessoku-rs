#![allow(clippy::many_single_char_names)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let mut dp: Vec<Vec<i64>> = vec![vec![0; w + 1]; h + 1];
    dp[0][0] = 1;
    for i in 1..h {
        if c[i][0] == '.' {
            dp[i][0] = dp[i - 1][0];
        } else {
            break;
        }
    }
    for i in 1..w {
        if c[0][i] == '.' {
            dp[0][i] = dp[0][i - 1];
        } else {
            break;
        }
    }
    for i in 1..h {
        for j in 1..w {
            if c[i][j] == '.' {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
    }
    println!("{:?}", dp[h - 1][w - 1]);
}
