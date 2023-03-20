#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut lr: [(i64, i64); n],
    }
    lr.sort_by(|a, b| a.1.cmp(&(b.1)));

    let mut current_time: i64 = 0;
    let mut answer: i64 = 0;
    for item in lr.iter() {
        if current_time <= item.0 {
            current_time = item.1;
            answer += 1;
        }
    }
    println!("{}", answer);
}
