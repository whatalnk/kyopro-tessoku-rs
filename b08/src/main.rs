use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    }
    let h = 10;
    let w = 10;
    let mut grid = vec![vec![0; w + 2]; h + 2];
    for pair in &xy {
        let (x, y) = pair;
        grid[*y][*x] += 1;
    }

    let mut grid2 = vec![vec![0; w + 2]; h + 2];
    for i in 0..h {
        for j in 1..=(w + 1) {
            grid2[i][j] = grid2[i][j - 1] + grid[i][j];
        }
    }

    for j in 0..w {
        for i in 1..=(h + 1) {
            grid2[i][j] += grid2[i - 1][j];
        }
    }

    for qq in &abcd {
        let (a, b, c, d) = qq;
        println!(
            "{}",
            grid2[*d][*c] - grid2[*d][*a - 1] - grid2[*b - 1][*c] + grid2[*b - 1][*a - 1]
        );
    }
}
