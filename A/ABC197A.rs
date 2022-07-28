use proconio::{input, marker::Chars};
use std::cmp;

fn main() {
    input! {
        s : String
    }

    let s_dash_head = &s[..1];
    let s_dash_body = &s[1..];

    println!("{}{}", s_dash_body, s_dash_head);
}
