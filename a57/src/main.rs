#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input!(n: usize, q: usize);
    input!(a: [usize; n]);
    let mut dp = vec![vec![0; 100_009]; 32];
    dp[0][1..=n].clone_from_slice(&a[0..n]);
    for d in 1..30 {
        for i in 1..=n {
            dp[d][i] = dp[d - 1][dp[d - 1][i]];
        }
    }
    for _ in 1..=q {
        input!(x: usize, y: usize);
        let mut cur = x;
        for d in 0..=29 {
            let dd = 29 - d;
            if (y / (1 << dd)) % 2 != 0 {
                cur = dp[dd][cur];
            }
        }
        println!("{}", cur);
    }
}
