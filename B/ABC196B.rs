use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        x: Chars,
    }

    let mut ans = String::new();

    for c in x {
        if c == '.' {
            break;
        } else {
            ans += &c.to_string();
        }
    }

    println!("{}", ans);
}
