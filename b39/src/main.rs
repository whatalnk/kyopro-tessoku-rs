#![allow(clippy::many_single_char_names)]

use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        d: usize,
        mut xy: [(usize, i64); n],
    }
    xy.sort_by(|a, b| a.0.cmp(&(b.0)));
    let mut cur = 0;
    let mut bh = BinaryHeap::new();
    let mut ans = 0;
    for i in 0..=d {
        loop {
            if cur < n && xy[cur].0 <= i {
                bh.push(xy[cur].1);
                cur += 1;
            } else {
                break;
            }
        }
        if let Some(y) = bh.pop() {
            ans += y;
        }
    }
    println!("{}", ans);
}
