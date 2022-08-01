use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        n : usize,
        m : usize,
        uv: [(Usize1, Usize1); m]
    }

    let mut ans = 0;
    let mut adj = vec![vec![false; n]; n];
    for (u, v) in uv {
        adj[u][v] = true;
        adj[v][u] = true;
    }

    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                if adj[i][j] && adj[j][k] && adj[k][i] {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans)
}
