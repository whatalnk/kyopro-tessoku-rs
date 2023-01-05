use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64
    }
    for i in a..=b {
        if 100 % i == 0 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
