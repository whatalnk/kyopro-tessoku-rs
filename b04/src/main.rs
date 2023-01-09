use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: Chars,
    }
    let mut ans = 0;
    let l = n.len();
    for i in 1..=l {
        if n[i - 1] == '1' {
            ans += 1 << (l - i);
        }
    }
    println!("{}", ans);
}
