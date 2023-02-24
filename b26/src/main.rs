#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut p = vec![true; n + 1];
    p[1] = false;
    let mut i = 2;
    loop {
        if i * i > n {
            break;
        }
        if p[i] {
            let mut ii = i + i;
            loop {
                if ii > n {
                    break;
                } else {
                    p[ii] = false;
                    ii += i
                }
            }
        }
        i += 1;
    }
    for (i, x) in p.iter().enumerate().skip(1) {
        if *x {
            println!("{}", i);
        }
    }
}
