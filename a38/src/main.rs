#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lrh: [(usize, usize, i64); n],
    }
    let mut lim = vec![24; d + 1];
    for lrhi in lrh.iter() {
        let (l, r, mut h) = *lrhi;
        for limj in lim.iter_mut().take(r + 1).skip(l) {
            *limj = *limj.min(&mut h);
        }
    }
    let mut ans = 0;
    for limi in lim.iter().take(d + 1).skip(1) {
        ans += limi;
    }

    println!("{}", ans);
}
