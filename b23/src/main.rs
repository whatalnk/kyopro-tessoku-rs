#![allow(clippy::many_single_char_names)]
#![allow(clippy::needless_range_loop)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n],
    }
    let mut d = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            let (x1, y1) = xy[i];
            let (x2, y2) = xy[j];
            let v = ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt();
            d[i][j] = v;
            d[j][i] = v;
        }
    }
    let max_n = 15;
    let inf: f64 = 1_000_000.0;
    let mut dp = vec![vec![inf; max_n]; 1 << max_n];
    dp[(1 << n) - 1][0] = 0.0;
    for s in 0..((1 << n) - 1) {
        let ss = (1 << n) - 2 - s;
        for v in 0..n {
            for u in 0..n {
                if (ss >> u & 1) == 0 {
                    dp[ss][v] = dp[ss][v].min(dp[ss | (1 << u)][u] + d[v][u]);
                }
            }
        }
    }
    println!("{}", dp[0][0]);
}
