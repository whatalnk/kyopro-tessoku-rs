#![allow(clippy::many_single_char_names)]

use proconio::input;

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
            dat: vec![0; siz * 2],
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
            return -1_000_000_000;
        }
        if l <= a && b <= r {
            return self.dat[u];
        }
        let m = (a + b) / 2;
        let ans1 = self.query(l, r, a, m, u * 2);
        let ans2 = self.query(l, r, m, b, u * 2 + 1);
        ans1.max(ans2)
    }
}

fn main() {
    input!(n: usize, q: usize);
    let mut seg_tree = SegmentTree::new(n);
    for _ in 0..q {
        input!(t: usize);
        match t {
            1 => {
                input!(p: usize, x: i64);
                seg_tree.update(p, x);
            }
            _ => {
                input!(l: usize, r: usize);
                let ans = seg_tree.query(l, r, 1, seg_tree.siz + 1, 1);
                println!("{}", ans);
            }
        }
    }
}
