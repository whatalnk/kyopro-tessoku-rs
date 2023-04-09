#![allow(clippy::many_single_char_names)]

use proconio::input;
use std::collections::HashMap;

fn main() {
    input!(n: usize);
    let mut hm = HashMap::<i64, i64>::new();
    for _ in 0..n {
        input!(a: i64);
        let e = hm.entry(a).or_insert(0);
        *e += 1;
    }
    let mut ans = 0;
    for v in hm.values() {
        ans += v * (v - 1) / 2;
    }
    println!("{}", ans);
}
