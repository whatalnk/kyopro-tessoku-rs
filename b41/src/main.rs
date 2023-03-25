#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }
    if x == 1 && y == 1 {
        println!("0");
        return;
    }
    let mut answer = vec![];
    answer.push((x, y));
    let mut xx = x;
    let mut yy = y;
    loop {
        if xx < yy {
            yy -= xx;
        } else {
            xx -= yy;
        }
        if xx == 1 && yy == 1 {
            break;
        }
        answer.push((xx, yy));
    }
    println!("{}", answer.len());
    for item in answer.iter().rev() {
        println!("{} {}", item.0, item.1);
    }
}
