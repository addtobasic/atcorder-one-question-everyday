use proconio::input;

fn main() {
    input! {
        x: isize,
    }

    println!("{}", calc_relu(x));
}

fn calc_relu(x: isize) -> isize {
    if x >= 0 {
        return x;
    } else {
        return 0;
    }
}
