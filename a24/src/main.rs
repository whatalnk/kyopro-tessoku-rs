#![allow(clippy::many_single_char_names)]

use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut dp = vec![0; n + 1];
    let mut length = 0;
    let mut l = vec![500_009; n + 1];
    l[0] = 0;
    for i in 0..n {
        let pos = l.lower_bound(&a[i]);
        dp[i] = pos;

        l[dp[i]] = a[i];
        if dp[i] > length {
            length += 1;
        }
    }
    println!("{}", length);
}
