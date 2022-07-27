use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
    }

    println!("{}", (2 * a + 100) - b);
}
