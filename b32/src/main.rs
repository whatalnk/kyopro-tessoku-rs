#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
    }
    let mut dp = vec![false; n + 9];
    for i in 0..=n {
        for j in 0..k {
            if i >= a[j] && !dp[i - a[j]] {
                dp[i] = true;
            }
        }
    }
    if dp[n] {
        println!("First");
    } else {
        println!("Second");
    }
}
