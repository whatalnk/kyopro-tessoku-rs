#![allow(clippy::many_single_char_names)]

use proconio::input;
use std::collections::HashSet;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut t = a
        .to_vec()
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<i64>>();
    t.sort();
    let mut b = vec![0; n];
    for i in 0..n {
        let pos = t.lower_bound(&a[i]);
        b[i] = pos + 1;
    }
    println!(
        "{}",
        b.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
