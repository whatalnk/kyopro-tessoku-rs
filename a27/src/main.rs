#![allow(clippy::many_single_char_names)]

use proconio::input;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while a >= 1 && b >= 1 {
        if a >= b {
            a = a % b;
        } else {
            b = b % a;
        }
    }
    if a != 0 {
        return a;
    } else {
        return b;
    }
}

fn main() {
    input! {
        a: i64,
        b: i64,
    }
    println!("{}", gcd(a, b));
}
