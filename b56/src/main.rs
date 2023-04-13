#![allow(clippy::many_single_char_names)]

use proconio::input;
use proconio::marker::Chars;

struct StringHash {
    h: Vec<i64>,
    md: i64,
}
impl StringHash {
    fn hash_value(&self, l: usize, r: usize, power100: &[i64]) -> i64 {
        let mut val = self.h[r] - (self.h[l - 1] * power100[r - l + 1] % self.md);
        if val < 0 {
            val += self.md;
        }
        val
    }
}

fn main() {
    input!(n: usize, q: usize);
    input!(s: Chars);
    let mut l = vec![0; 100_009];
    let mut r = vec![0; 100_009];
    let md: i64 = 2_147_483_647;
    for i in 1..=q {
        input!(li: usize, ri: usize);
        l[i] = li;
        r[i] = ri;
    }
    let mut t1 = vec![0; 100_009];
    let mut t2 = vec![0; 100_009];
    for i in 1..=n {
        t1[i] = (s[i - 1] as u8 - b'a' + 1) as i64;
        t2[i] = (s[n - i] as u8 - b'a' + 1) as i64;
    }
    let mut power100 = vec![0; 100_009];
    power100[0] = 1;
    for i in 1..=n {
        power100[i] = 100 * power100[i - 1] % md;
    }
    let mut h1 = vec![0; 100_009];
    let mut h2 = vec![0; 100_009];
    for i in 1..=n {
        h1[i] = (100 * h1[i - 1] + t1[i]) % md;
        h2[i] = (100 * h2[i - 1] + t2[i]) % md;
    }
    let string_hash1 = StringHash { h: h1, md };
    let string_hash2 = StringHash { h: h2, md };
    for i in 1..=q {
        let hash1 = string_hash1.hash_value(l[i], r[i], &power100);
        let hash2 = string_hash2.hash_value(n - r[i] + 1, n - l[i] + 1, &power100);
        if hash1 == hash2 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
