#![allow(clippy::many_single_char_names)]

use proconio::input;
use proconio::marker::Chars;

struct StringHash {
    h: Vec<i64>,
    power100: Vec<i64>,
    md: i64,
}
impl StringHash {
    fn hash_value(&self, l: usize, r: usize) -> i64 {
        let mut val = self.h[r] - (self.h[l - 1] * self.power100[r - l + 1] % self.md);
        if val < 0 {
            val += self.md;
        }
        val
    }
}

fn main() {
    input!(n: usize, q: usize);
    input!(s: Chars);
    let mut a = vec![0; 200_009];
    let mut b = vec![0; 200_009];
    let mut c = vec![0; 200_009];
    let mut d = vec![0; 200_009];
    let md: i64 = 2_147_483_647;
    for i in 1..=q {
        input!(ai: usize, bi: usize, ci: usize, di: usize);
        a[i] = ai;
        b[i] = bi;
        c[i] = ci;
        d[i] = di;
    }
    let mut t = vec![0; 200_009];
    for i in 1..=n {
        t[i] = (s[i - 1] as u8 - b'a' + 1) as i64;
    }
    let mut power100 = vec![0; 200_009];
    power100[0] = 1;
    for i in 1..=n {
        power100[i] = 100 * power100[i - 1] % md;
    }
    let mut h = vec![0; 200_009];
    for i in 1..=n {
        h[i] = (100 * h[i - 1] + t[i]) % md;
    }
    let string_hash = StringHash { h, power100, md };
    for i in 1..=q {
        let hash1 = string_hash.hash_value(a[i], b[i]);
        let hash2 = string_hash.hash_value(c[i], d[i]);
        if hash1 == hash2 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
