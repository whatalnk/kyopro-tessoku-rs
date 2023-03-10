#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut xor_sum = a[0];
    for ai in a.iter().skip(1) {
        xor_sum ^= ai;
    }
    if xor_sum == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
