#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }
    let mut answer = 0;
    let mut asum = 0;
    let mut bsum = 0;
    for i in 0..n {
        if ab[i].0 >= 0 {
            asum += ab[i].0;
            bsum += ab[i].1;
        }
    }
    answer = answer.max(asum.abs() + bsum.abs());

    let mut asum = 0;
    let mut bsum = 0;
    for i in 0..n {
        if ab[i].1 >= 0 {
            asum += ab[i].0;
            bsum += ab[i].1;
        }
    }
    answer = answer.max(asum.abs() + bsum.abs());

    let mut asum = 0;
    let mut bsum = 0;
    for i in 0..n {
        if ab[i].0 <= 0 {
            asum += ab[i].0;
            bsum += ab[i].1;
        }
    }
    answer = answer.max(asum.abs() + bsum.abs());

    let mut asum = 0;
    let mut bsum = 0;
    for i in 0..n {
        if ab[i].1 <= 0 {
            asum += ab[i].0;
            bsum += ab[i].1;
        }
    }
    answer = answer.max(asum.abs() + bsum.abs());
    println!("{}", answer);
}
