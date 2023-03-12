#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        _x: usize,
        _y: usize,
        a: [usize; n],
    }
    let grundy = vec![0, 0, 1, 1, 2];
    let mut xor_sum = 0;
    for i in 0..n {
        xor_sum ^= grundy[a[i] % 5];
    }
    if xor_sum != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
