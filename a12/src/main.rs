#![allow(clippy::many_single_char_names)]

use proconio::input;

fn check(x: usize, a: &Vec<i64>, k: i64) -> bool {
    let mut s = 0;
    for ai in a {
        s += x as i64 / ai;
    }
    if s >= k {
        return true;
    } else {
        return false;
    }
}

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }
    let mut l = 1;
    let mut r = 1000000000;
    while l < r {
        let m = (l + r) / 2;
        let ans = check(m, &a, k);
        if !ans {
            l = m + 1;
        } else {
            r = m;
        }
    }
    println!("{}", l);
}
