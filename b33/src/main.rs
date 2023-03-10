#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        _h: usize,
        _w: usize,
        ab: [(usize, usize); n],
    }
    let mut xor_sum = 0;
    for abi in ab.iter() {
        xor_sum ^= abi.0 - 1;
    }
    for abi in ab.iter() {
        xor_sum ^= abi.1 - 1;
    }
    if xor_sum == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
