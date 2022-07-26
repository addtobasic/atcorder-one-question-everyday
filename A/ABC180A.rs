use proconio::input;

fn main() {
    input! {
        n: isize,
        a: isize,
        b: isize,
    }

    println!("{}", n - a + b);
}
