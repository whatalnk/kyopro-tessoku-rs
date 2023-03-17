#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let mut r = vec![vec![0; 10]; 18];
    let mut power10 = vec![0; 18];
    power10[0] = 1;
    for i in 1..=16 {
        power10[i] = 10 * power10[i - 1];
    }
    for i in 0..=14 {
        let digit = (n / power10[i]) % 10;
        for j in 0..10 {
            if (j as i64) < digit {
                r[i][j] = (n / power10[i + 1]) * power10[i] + power10[i];
            }
            if (j as i64) == digit {
                r[i][j] = (n / power10[i + 1]) * power10[i] + (n % power10[i]) + 1;
            }
            if (j as i64) > digit {
                r[i][j] = (n / power10[i + 1]) * power10[i];
            }
        }
    }
    let mut answer = 0;
    for row in r.iter().take(16) {
        for (j, cell) in row.iter().enumerate().take(10) {
            answer += (j as i64) * cell;
        }
    }
    println!("{}", answer);
}
