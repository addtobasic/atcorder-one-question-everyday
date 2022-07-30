use proconio::{input, marker::Chars};
use std::cmp;

fn main() {
    input! {
        n : usize,
        s : [String; n],
    }

    let mut ac = 0;
    let mut wa = 0;
    let mut tle = 0;
    let mut re = 0;

    for i in 0..n {
        if s[i] == "AC" {
            ac += 1;
        }

        else if s[i] == "WA" {
            wa += 1;
        }

        else if s[i] == "TLE" {
            tle += 1;
        }

        else if s[i] == "RE" {
            re += 1;
        }
    }

    println!("AC x {}", ac);
    println!("WA x {}", wa);
    println!("TLE x {}", tle);
    println!("RE x {}", re);
}
