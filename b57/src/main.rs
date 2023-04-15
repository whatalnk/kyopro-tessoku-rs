#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {n: usize, k: usize}
    let mut dp = vec![vec![0; 300_009]; 32];
    for i in 1..=n {
        dp[0][i] = i;
        let mut num = i;
        while num > 0 {
            dp[0][i] -= num % 10;
            num /= 10;
        }
    }
    for d in 1..=29 {
        for i in 1..=n {
            dp[d][i] = dp[d - 1][dp[d - 1][i]];
        }
    }
    for i in 1..=n {
        let mut cur = i;
        for d in 0..=29 {
            let dd = 29 - d;
            if k / (1 << dd) % 2 != 0 {
                cur = dp[dd][cur];
            }
        }
        println!("{}", cur);
    }
}
