#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }
    let mut dp = vec![vec![false; 10009]; 69];
    dp[0][0] = true;
    for i in 1..=n {
        for j in 0..=s {
            if j < a[i - 1] {
                if dp[i - 1][j] {
                    dp[i][j] = true;
                } else {
                    dp[i][j] = false;
                }
            }
            if j >= a[i - 1] {
                if dp[i - 1][j] || dp[i - 1][j - a[i - 1]] {
                    dp[i][j] = true;
                } else {
                    dp[i][j] = false;
                }
            }
        }
    }
    if !dp[n][s] {
        println!("-1");
        return;
    }
    let mut place = s;
    let mut route = vec![];
    for i in 0..n {
        let ii = n - i;
        if dp[ii - 1][place] {
            continue;
        } else {
            place -= a[ii - 1];
            route.push(ii);
        }
    }
    route.reverse();
    println!("{}", route.len());
    println!(
        "{}",
        route
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
