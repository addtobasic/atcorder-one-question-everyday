use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: Usize1,
        y: Usize1,
        s: [Chars; h],
    }

    let mut ans = 1;

    for i in x + 1..h {
        if s[i][y] == '#' {
            break;
        }

        ans += 1;
    }

    for i in (0..x).rev() {
        if s[i][y] == '#' {
            break;
        }

        ans += 1;
    }

    for j in y + 1..w {
        if s[x][j] == '#' {
            break;
        }

        ans += 1;
    }

    for j in (0..y).rev() {
        if s[x][j] == '#' {
            break;
        }

        ans += 1;
    }

    println!("{}", ans);
}
