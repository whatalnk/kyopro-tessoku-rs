use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    }
    let h = 1500;
    let w = 1500;
    let mut grid: Vec<Vec<i64>> = vec![vec![0; w + 9]; h + 9];
    for pair in &xy {
        let (x, y) = pair;
        grid[*x][*y] += 1;
    }

    let mut grid2: Vec<Vec<i64>> = vec![vec![0; w + 9]; h + 9];
    for i in 1..=h {
        for j in 1..=w {
            grid2[i][j] = grid2[i][j - 1] + grid[i][j];
        }
    }

    for j in 1..=w {
        for i in 1..=h {
            grid2[i][j] += grid2[i - 1][j];
        }
    }

    for qq in &abcd {
        let (a, b, c, d) = qq;
        println!(
            "{}",
            grid2[*c][*d] - grid2[*a - 1][*d] - grid2[*c][*b - 1] + grid2[*a - 1][*b - 1]
        );
    }
}
