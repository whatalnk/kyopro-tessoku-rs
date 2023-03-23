#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut c: Vec<i64> = vec![0; 101];
    for item in a.iter() {
        c[*item % 100] += 1;
    }
    let mut answer = 0;
    for i in 1..50 {
        answer += c[i] * c[100 - i];
    }
    answer += c[0] * (c[0] - 1) / 2;
    answer += c[50] * (c[50] - 1) / 2;
    println!("{}", answer);
}
