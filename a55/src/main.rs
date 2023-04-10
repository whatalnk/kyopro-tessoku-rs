#![allow(clippy::many_single_char_names)]

use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input!(q: usize);
    let mut bts = BTreeSet::new();
    for _ in 0..q {
        input!(t: usize);
        match t {
            1 => {
                input!(x: i64);
                bts.insert(x);
            }
            2 => {
                input!(x: i64);
                bts.remove(&x);
            }
            _ => {
                input!(x: i64);
                if let Some(e) = bts.range(x..).next() {
                    println!("{}", e);
                } else {
                    println!("-1");
                }
            }
        }
    }
}
