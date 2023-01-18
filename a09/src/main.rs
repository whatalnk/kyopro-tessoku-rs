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
    for i in 0..h {
        for j in 0..w {
            mm[i + 1][j + 1] = mm[i + 1][j] + m[i][j];
        }
    }
    for j in 0..w {
        for i in 0..h {
            mm[i + 1][j + 1] += mm[i][j + 1];
        }
    }
    for row in mm.iter().skip(2) {
        println!(
            "{}",
            row.iter()
                .skip(2)
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
