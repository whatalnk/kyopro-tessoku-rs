use proconio::input;

fn main() {
    input! {
      n: usize,
      a: [i64; n],
    }
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if i != j && j != k && k != i && a[i] + a[j] + a[k] == 1000 {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
