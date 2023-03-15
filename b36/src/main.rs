#![allow(clippy::many_single_char_names)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        k: usize,
        s: Chars,
    }
    let mut c_on = 0;
    for l in s.iter() {
        if l == &'1' {
            c_on += 1;
        }
    }
    if k % 2 == c_on % 2 {
        println!("Yes");
    } else {
        println!("No")
    }
}
