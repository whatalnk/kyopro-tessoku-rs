#![allow(clippy::many_single_char_names)]

use proconio::input;

fn power(a: i128, b: i128, m: i128) -> i128 {
    let mut p = a;
    let mut ans = 1;
    for i in 0..100 {
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
        a: i128,
        b: i128,
    }
    println!("{}", power(a, b, 1_000_000_007));
}
