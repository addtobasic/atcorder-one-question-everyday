use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        mut n: usize
    }

    if n == 0 {
        println!("Yes");
        return;
    }

    while n % 10 == 0 {
        n /= 10;
    }

    let text = n.to_string();
    let rev = text.chars().rev().collect::<String>();

    if text == rev {
        println!("Yes");
    } else {
        println!("No");
    }
}
