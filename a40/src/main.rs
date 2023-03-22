#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut c: Vec<i64> = vec![0; 101];
    for item in a.iter() {
        c[*item] += 1;
    }
    let mut answer = 0;
    for item in c.iter() {
        answer += item * (item - 1) * (item - 2) / 6;
    }
    println!("{}", answer);
}
