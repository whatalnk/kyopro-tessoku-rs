#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }
    let mut answer = 0;
    let s1 = vec![1, 1, -1, -1];
    let s2 = vec![1, -1, -1, -1];
    for i in 0..4 {
        let ss1 = s1[i];
        let ss2 = s2[i];
        let mut asum = 0;
        let mut bsum = 0;
        for item in ab.iter() {
            if ss1 * item.0 + ss2 * item.1 > 0 {
                asum += item.0;
                bsum += item.1;
            }
        }
        answer = answer.max(asum.abs() + bsum.abs());
    }
    println!("{}", answer);
}
