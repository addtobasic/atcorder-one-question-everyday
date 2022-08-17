use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        s: Chars,
    }

    let mut is_illegible = true;

    for i in 0..s.len() {
        if s[i].is_uppercase() && (i + 1) % 2 == 1 {
            is_illegible = false;
        }
        if s[i].is_lowercase() && (i + 1) % 2 == 0 {
            is_illegible = false;
        }
    }

    println!("{}", if is_illegible { "Yes" } else { "No" });
}
