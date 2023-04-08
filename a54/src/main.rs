#![allow(clippy::many_single_char_names)]

use proconio::input;
use std::collections::HashMap;

fn main() {
    input!(q: usize);
    let mut hm = HashMap::new();
    for _ in 0..q {
        input!(t: usize);
        match t {
            1 => {
                input!(x: String, y: i64);
                hm.insert(x, y);
            }
            _ => {
                input!(x: String);
                if let Some(e) = hm.get(&x) {
                    println!("{}", e);
                }
            }
        }
    }
}
