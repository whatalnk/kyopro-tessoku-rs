#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        ab: [(i64, i64); n],
    }
    let mut answer = 0;
    for aa in 1..=100 {
        for bb in 1..=100 {
            let mut score = 0;
            for item in ab.iter() {
                if item.0 >= aa && item.0 <= aa + k && item.1 >= bb && item.1 <= bb + k {
                    score += 1;
                }
            }
            answer = answer.max(score);
        }
    }
    println!("{}", answer);
}
