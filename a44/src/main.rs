#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input!(n: usize, q: usize);
    let mut a = (0..=n).collect::<Vec<usize>>();
    let mut state = 1;
    for _ in 0..q {
        input!(t: i64);
        if t == 1 {
            input!(x: usize, y: usize);
            if state == 1 {
                a[x] = y;
            } else {
                a[n + 1 - x] = y;
            }
        } else if t == 2 {
            if state == 1 {
                state = 2;
            } else {
                state = 1;
            }
        } else {
            input!(x: usize);
            if state == 1 {
                println!("{}", a[x]);
            } else {
                println!("{}", a[n + 1 - x]);
            }
        }
    }
}
