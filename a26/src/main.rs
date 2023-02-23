#![allow(clippy::many_single_char_names)]

use proconio::input;

fn is_prime(x: i64) -> bool {
    let mut i = 2;
    loop {
        if i * i > x {
            break;
        }
        if x % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}

fn main() {
    input! {
        q: usize,
        x: [i64; q],
    }
    for xi in x.iter() {
        if is_prime(*xi) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
