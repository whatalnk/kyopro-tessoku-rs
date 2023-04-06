#![allow(clippy::many_single_char_names)]

use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        x: usize,
        mut a: Chars,
    }
    let mut queue = VecDeque::new();
    queue.push_back(x - 1);
    a[x - 1] = '@';
    while let Some(pos) = queue.pop_front() {
        if pos >= 1 && a[pos - 1] == '.' {
            a[pos - 1] = '@';
            queue.push_back(pos - 1);
        }
        if pos + 1 < n && a[pos + 1] == '.' {
            a[pos + 1] = '@';
            queue.push_back(pos + 1);
        }
    }
    println!("{}", a.iter().collect::<String>());
}
