use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        n: usize,
        x: usize,
        s: Chars,
    }

    let mut ans = 0;
    ans = x;

    for i in 0..n {
        if s[i] == 'x' {
            if ans != 0 {
                ans -= 1;
            }
        } else {
            ans += 1
        }
    }
    println!("{}", ans);
}
