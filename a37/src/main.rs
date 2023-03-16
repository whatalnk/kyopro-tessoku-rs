#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: usize,
        a: [i64; n],
        c: [i64; m],
    }
    let mut ans = 0;
    for ai in a.iter() {
        ans += ai * m as i64;
    }
    ans += (b * n * m) as i64;
    for ci in c.iter() {
        ans += ci * n as i64;
    }
    println!("{}", ans);
}
