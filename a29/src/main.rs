#![allow(clippy::many_single_char_names)]

use proconio::input;

fn power(a: i64, b: i64, m: i64) -> i64 {
    let mut p = a;
    let mut ans = 1;
    for i in 0..30 {
        let d = 1 << i;
        if (b / d) % 2 == 1 {
            ans = (ans * p) % m;
        }
        p = (p * p) % m;
    }
    ans
}

fn main() {
    input! {
        a: i64,
        b: i64,
    }
    println!("{}", power(a, b, 1_000_000_007));
}
