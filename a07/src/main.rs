use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize, usize); n],
    }
    let mut b = vec![0; d + 2];
    for x in &lr {
        let (l, r) = x;
        b[*l] += 1;
        b[*r + 1] -= 1;
    }
    let mut ans = vec![0; d + 2];
    for i in 0..d {
        ans[i + 1] = ans[i] + b[i + 1];
    }
    for x in ans.iter().take(d + 1).skip(1) {
        println!("{}", x);
    }
}
