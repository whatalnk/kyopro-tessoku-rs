#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let c3 = n / 3;
    let c5 = n / 5;
    let c7 = n / 7;
    let c35 = n / 15;
    let c57 = n / 35;
    let c37 = n / 21;
    let c357 = n / 105;
    println!("{}", c3 + c5 + c7 - c35 - c57 - c37 + c357);
}
