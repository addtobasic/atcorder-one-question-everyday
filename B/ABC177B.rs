use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        s : Chars,
        t : Chars,
    }

    let mut ans = t.len();

    for i in 0..=s.len() - t.len() {
        let mut diff = 0;
        for j in 0..t.len() {
            if t[j] != s[i + j] {
                diff += 1;
            }
        }
        ans = cmp::min(ans, diff);
    }

    println!("{}", ans);
}
