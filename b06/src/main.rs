use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        q: usize,
        lr: [(usize, usize); q],
    }
    let mut wins = vec![0; n + 1];
    let mut loses = vec![0; n + 1];
    for i in 0..n {
        if a[i] == 1 {
            wins[i + 1] = wins[i] + 1;
            loses[i + 1] = loses[i];
        } else {
            wins[i + 1] = wins[i];
            loses[i + 1] = loses[i] + 1;
        }
    }
    for x in &lr {
        let (l, r) = x;
        let nwins = wins[*r] - wins[*l - 1];
        let nloses = loses[*r] - loses[*l - 1];
        match nwins.cmp(&nloses) {
            Ordering::Less => {
                println!("lose");
            }
            Ordering::Greater => {
                println!("win");
            }
            Ordering::Equal => {
                println!("draw");
            }
        }
    }
}
