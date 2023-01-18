#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }
    let mut m = vec![vec![0; w + 2]; h + 2];
    for qq in &abcd {
        let (a, b, c, d) = qq;
        m[*a][*b] += 1;
        m[*c + 1][*d + 1] += 1;
        m[*c + 1][*b] -= 1;
        m[*a][*d + 1] -= 1;
    }
    let mut mm = vec![vec![0; w + 2]; h + 2];
    for i in 1..=h {
        for j in 1..=w {
            mm[i][j] = mm[i][j - 1] + m[i][j];
        }
    }
    for j in 1..=w {
        for i in 1..=h {
            mm[i][j] += mm[i - 1][j];
        }
    }
    for row in mm.iter().take(h + 1).skip(1) {
        println!(
            "{}",
            row.iter()
                .take(w + 1)
                .skip(1)
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
