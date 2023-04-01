#![allow(clippy::many_single_char_names)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        c: char,
        a: Chars,
    }
    let mut score = 0;
    for item in a.iter() {
        if item == &'W' {
            score += 0;
        } else if item == &'B' {
            score += 1;
        } else {
            score += 2;
        }
    }
    if (score % 3 == 0 && c == 'W') || (score % 3 == 1 && c == 'B') || (score % 3 == 2 && c == 'R')
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
