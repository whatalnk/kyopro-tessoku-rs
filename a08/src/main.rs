use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[i64; w]; h],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    }
    let mut y = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            y[i + 1][j + 1] = y[i + 1][j] + x[i][j];
        }
    }
    for j in 0..w {
        for i in 0..h {
            y[i + 1][j + 1] += y[i][j + 1];
        }
    }
    for qq in &abcd {
        let (a, b, c, d) = qq;
        println!(
            "{}",
            y[*c][*d] - y[*a - 1][*d] - y[*c][*b - 1] + y[*a - 1][*b - 1]
        );
    }
}
