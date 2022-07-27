use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    }

    let ans = a * d - b * c;

    println!("{}", ans);
}
