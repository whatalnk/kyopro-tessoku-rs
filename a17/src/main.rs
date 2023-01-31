#![allow(clippy::many_single_char_names)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n-1],
        b: [i64; n-2],
    }
    let mut dp = vec![10_000_001; n + 1];
    dp[1] = 0;
    dp[2] = a[0];
    for i in 3..=n {
        dp[i] = (dp[i - 1] + a[i - 2]).min(dp[i - 2] + b[i - 3]);
    }
    let mut place = n;
    let mut route = vec![];
    loop {
        route.push(place);
        if place == 1 {
            break;
        }
        if dp[place - 1] + a[place - 2] == dp[place] {
            place -= 1;
        } else {
            place -= 2;
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
