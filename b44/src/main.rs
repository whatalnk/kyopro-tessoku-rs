#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input!(n: usize);
    input!(a: [[i64; n]; n]);
    input!(q: usize);
    let mut b = (0..=n).collect::<Vec<usize>>();
    for _ in 0..q {
        input!(t: usize, x: usize, y: usize);
        if t == 1 {
            b.swap(x, y);
        } else {
            println!("{}", a[b[x] - 1][y - 1]);
        }
    }
}
