#![allow(clippy::many_single_char_names)]

use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        mut xy: [(i64, i64); n],
    }
    xy.sort_by_key(|&(x, y)| (x, !y));

    let a = xy.iter().map(|e| e.1).collect::<Vec<i64>>();

    let mut length = 0;
    let mut l = vec![500_009; n + 1];
    l[0] = 0;
    for ai in a.iter() {
        let pos = l.lower_bound(ai);
        l[pos] = *ai;
        if pos > length {
            length += 1;
        }
    }
    println!("{}", length);
}
