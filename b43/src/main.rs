#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: i64,
        a: [usize; m],
    }
    let mut cnt = vec![m; n + 1];
    for item in a.iter() {
        cnt[*item] -= 1;
    }
    for item in cnt.iter().skip(1) {
        println!("{}", item);
    }
}
