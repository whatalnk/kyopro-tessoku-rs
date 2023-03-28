#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        l: i64,
        ab: [(i64, char); n],
    }
    let mut answer = 0;
    for item in ab.iter() {
        if item.1 == 'E' {
            answer = answer.max(l - item.0);
        } else {
            answer = answer.max(item.0);
        }
    }
    println!("{}", answer);
}
