#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let c3 = n / 3;
    let c5 = n / 5;
    let c35 = n / 15;
    println!("{}", c3 + c5 - c35);
}
