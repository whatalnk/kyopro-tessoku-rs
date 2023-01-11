use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        lr: [(usize, usize); q],
    }
    let mut b = vec![0; n + 1];
    for i in 0..n {
        b[i + 1] = b[i] + a[i];
    }
    for x in &lr {
        let (l, r) = x;
        println!("{}", b[*r] - b[*l - 1]);
    }
}
