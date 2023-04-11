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
            _ => {
                input!(x: i64);
                let mut ans = 1_000_000_000;
                if let Some(e) = bts.range(x..).next() {
                    ans = ans.min((x - e).abs());
                }
                if let Some(e) = bts.range(..=x).rev().next() {
                    ans = ans.min((x - e).abs());
                }
                if ans == 1_000_000_000 {
                    ans = -1;
                }
                println!("{}", ans);
            }
        }
    }
}
