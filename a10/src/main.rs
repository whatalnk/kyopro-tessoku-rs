#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        d: usize,
        lr: [(usize, usize); d],
    }
    let mut vp = vec![0; n + 2];
    vp[1] = a[0];
    for i in 1..n {
        vp[i + 1] = vp[i].max(a[i]);
    }

    let mut vq = vec![0; n + 2];
    vq[n] = a[n - 1];
    for i in 1..n {
        vq[n - i] = vq[n - i + 1].max(a[n - i - 1]);
    }
    for i in 0..d {
        let (l, r) = lr[i];
        println!("{}", vp[l - 1].max(vq[r + 1]));
    }
}
