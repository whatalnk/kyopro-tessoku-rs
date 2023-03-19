#![allow(clippy::many_single_char_names)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut streak1 = 1;
    let mut ret1 = vec![0; n + 1];
    for i in 0..n - 1 {
        if s[i] == 'A' {
            streak1 += 1;
        } else {
            streak1 = 1;
        }
        ret1[i + 1] = streak1;
    }
    let mut streak2 = 1;
    let mut ret2 = vec![0; n + 1];
    for i in 0..n - 1 {
        let ii = n - 2 - i;
        if s[ii] == 'A' {
            streak2 = 1;
        } else {
            streak2 += 1;
        }
        ret2[ii] = streak2;
    }
    let mut ans = 0;
    for i in 0..n {
        ans += ret1[i].max(ret2[i]);
    }
    println!("{}", ans);
}
