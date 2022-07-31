use proconio::{input, marker::Chars};
use std::cmp;
use std::num;

fn main() {
    input! {
        n : Chars,
    }

    let ans: usize = n
        .into_iter()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .sum();

    let is_multiple_nine = ans % 9 == 0;

    println!("{}", if is_multiple_nine { "Yes" } else { "No" });
}
