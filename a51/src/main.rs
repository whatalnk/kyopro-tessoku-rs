#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input!(q: usize);
    let mut stack = vec![];
    for _ in 0..q {
        input!(t: usize);
        match t {
            1 => {
                input!(s: String);
                stack.push(s);
            }
            2 => {
                println!("{}", stack.last().unwrap());
            }
            _ => {
                stack.pop();
            }
        }
    }
}
