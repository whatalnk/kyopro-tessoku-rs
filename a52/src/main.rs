#![allow(clippy::many_single_char_names)]

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input!(q: usize);
    let mut queue = VecDeque::new();
    for _ in 0..q {
        input!(t: usize);
        match t {
            1 => {
                input!(s: String);
                queue.push_back(s);
            }
            2 => {
                println!("{}", queue.front().unwrap());
            }
            _ => {
                queue.pop_front();
            }
        }
    }
}
