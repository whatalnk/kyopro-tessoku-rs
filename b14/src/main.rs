#![allow(clippy::many_single_char_names)]

use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }
    let n1 = n / 2;
    let n2 = n - n1;
    let mut a1 = vec![0; 1 << n1];
    for (i, a1i) in a1.iter_mut().enumerate() {
        let mut s = 0;
        for (j, aj) in a.iter().enumerate().take(n1) {
            let d = 1 << j;
            if (i / d) % 2 == 1 {
                s += aj;
            }
        }
        *a1i = s;
    }

    let mut a2 = vec![0; 1 << n2];
    for (i, a2i) in a2.iter_mut().enumerate() {
        let mut s = 0;
        for (j, aj) in a.iter().skip(n1).enumerate() {
            let d = 1 << j;
            if (i / d) % 2 == 1 {
                s += aj;
            }
        }
        *a2i = s;
    }

    a2.sort();
    for a1i in a1.iter() {
        let pos = a2.lower_bound(&(k - a1i));
        if pos < a2.len() && a2[pos] == k - a1i {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
