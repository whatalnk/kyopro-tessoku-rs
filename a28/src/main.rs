#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input!(n: usize);
    let mut ans = 0;
    for _ in 0..n {
        input!(t: char, a: i64);
        if t == '+' {
            ans += a;
        } else if t == '-' {
            ans -= a;
        } else {
            ans *= a;
        }
        if ans < 0 {
            ans += 10000;
        }
        ans %= 10000;
        println!("{}", ans);
    }
}
