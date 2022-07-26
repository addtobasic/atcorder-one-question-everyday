use proconio::input;

fn main() {
    input! {
        x: isize,
    }

    if x == 1 {
        println!("{}", 0);
    } else {
        println!("{}", 1);
    }
}
