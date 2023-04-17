#![allow(clippy::many_single_char_names)]

use proconio::input;
use superslice::Ext;

struct SegmentTree {
    dat: Vec<i64>,
    siz: usize,
}
impl SegmentTree {
    fn new(n: usize) -> SegmentTree {
        let mut siz = 1;
        while siz < n {
            siz *= 2;
        }
        SegmentTree {
            dat: vec![0; 300_000],
            siz,
        }
    }
    fn update(&mut self, mut pos: usize, x: i64) {
        pos = pos + self.siz - 1;
        self.dat[pos] = x;
        while pos >= 2 {
            pos /= 2;
            self.dat[pos] = self.dat[pos * 2].max(self.dat[pos * 2 + 1]);
        }
    }
    fn query(&self, l: usize, r: usize, a: usize, b: usize, u: usize) -> i64 {
        if r <= a || b <= l {
            return 1_000_000_000;
        }
        if l <= a && b <= r {
            return self.dat[u];
        }
        let m = (a + b) / 2;
        let ans1 = self.query(l, r, a, m, u * 2);
        let ans2 = self.query(l, r, m, b, u * 2 + 1);
        ans1.min(ans2)
    }
}

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        x: [usize; n],
    }
    let mut seg_tree = SegmentTree::new(n);
    let mut dp = vec![0; 100_009];
    dp[1] = 0;
    seg_tree.update(1, 0);
    for i in 2..=n {
        let pos_l = if x[i - 1] > r {
            x.lower_bound(&(x[i - 1] - r))
        } else {
            0
        };
        let pos_r = if x[i - 1] + 1 > l {
            x.lower_bound(&(x[i - 1] - l + 1))
        } else {
            0
        };
        dp[i] = seg_tree.query(pos_l, pos_r + 1, 1, seg_tree.siz + 1, 1) + 1;
        seg_tree.update(i, dp[i]);
    }
    println!("{}", dp[n]);
}
