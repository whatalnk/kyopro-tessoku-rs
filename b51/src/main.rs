#![allow(clippy::many_single_char_names)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let mut stack = vec![];
    for (i, item) in s.iter().enumerate() {
        if item == &'(' {
            stack.push(i);
        } else {
            let l = stack.pop().unwrap();
            println!("{} {}", l + 1, i + 1);
        }
    }
}
