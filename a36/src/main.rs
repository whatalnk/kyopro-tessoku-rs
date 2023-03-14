#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    if k >= 2 * n - 2 && k % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
