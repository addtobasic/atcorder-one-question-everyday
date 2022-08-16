use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        n: usize,
        s: usize,
        d: usize,
        xy: [(usize, usize); n],
    }

    let mut is_can_do_damage = false;
    for (x, y) in xy {
        if x < s && y > d {
            is_can_do_damage = true;
        }
    }

    println!("{}", if is_can_do_damage { "Yes" } else { "No" });
}
