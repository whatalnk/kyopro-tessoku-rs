#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let mut dp = vec![false; n + 9];
    for i in 0..=n {
        if (i >= a && !dp[i - a]) || (i >= b && !dp[i - b]) {
            dp[i] = true;
        } else {
            dp[i] = false;
        }
    }
    if dp[n] {
        println!("First");
    } else {
        println!("Second");
    }
}
