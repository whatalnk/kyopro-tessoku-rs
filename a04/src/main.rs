use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let mut ans = vec![];
    for i in 1..=10 {
        let wari = 1 << (10 - i);
        ans.push((n / wari) % 2);
    }
    println!("{}", ans.iter().map(|x| x.to_string()).collect::<String>());
}
