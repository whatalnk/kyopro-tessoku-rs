#![allow(clippy::many_single_char_names)]

use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input!(q: usize);
    let mut pq = BinaryHeap::new();
    for _ in 0..q {
        input!(t: usize);
        match t {
            1 => {
                input!(x: i64);
                pq.push(Reverse(x));
            }
            2 => {
                println!("{}", pq.peek().unwrap().0);
            }
            _ => {
                pq.pop();
            }
        }
    }
}
