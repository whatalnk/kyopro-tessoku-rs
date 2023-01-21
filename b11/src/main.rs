#![allow(clippy::many_single_char_names)]

use proconio::input;
use superslice::Ext;

fn main() {
    input!(n: usize, mut a: [i64; n], q: i64);
    a.sort();
    for _ in 0..q {
        input!(x: i64);
        let r = a.lower_bound(&x);
        println!("{}", r);
    }
}
