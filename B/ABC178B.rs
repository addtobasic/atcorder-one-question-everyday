use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp;
use std::num;

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize
    }

    let ac = a * c;
    let ad = a * d;
    let bc = b * c;
    let bd = b * d;

    println!("{}", cmp::max(cmp::max(ac, ad), cmp::max(bc, bd)));
}
