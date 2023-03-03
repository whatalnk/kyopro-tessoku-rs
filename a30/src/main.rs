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

fn division(a: i64, b: i64, m: i64) -> i64 {
    (a * power(b, m - 2, m)) % m
}

fn main() {
    input! {
        n: i64,
        r: i64,
    }
    let m = 1_000_000_007;
    let mut a = 1;
    for i in 1..=n {
        a = (a * i) % m;
    }

    let mut b = 1;
    for i in 1..=r {
        b = (b * i) % m;
    }
    for i in 1..=(n - r) {
        b = (b * i) % m;
    }
    println!("{}", division(a, b, m));
}
