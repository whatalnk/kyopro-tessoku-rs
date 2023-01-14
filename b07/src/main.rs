use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n],
    }
    let mut b = vec![0; t + 2];
    for x in &lr {
        let (l, r) = x;
        b[*l] += 1;
        b[*r] -= 1;
    }
    for i in 0..=t {
        b[i + 1] += b[i];
    }
    for x in b.iter().take(t) {
        println!("{}", x);
    }
}
