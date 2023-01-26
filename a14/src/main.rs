#![allow(clippy::many_single_char_names)]

use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
        c: [i64; n],
        d: [i64; n],
    }
    let mut p = vec![0; n * n];
    let mut q = vec![0; n * n];
    for i in 0..n {
        for j in 0..n {
            p[i * n + j] = a[i] + b[j];
            q[i * n + j] = c[i] + d[j];
        }
    }
    q.sort();
    for pi in p.iter() {
        let pos1 = q.lower_bound(&(k - pi));
        if pos1 < n * n && q[pos1] == k - pi {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
