use proconio::input;

fn main() {
    input! {
        n: usize,
        mut d: [usize; n]
    }
    d.sort();
    let mut count = 0;
    let mut tmp = 0;

    for i in 0..n {
        if d[i] > tmp {
            count += 1;
            tmp = d[i];
        }
    }

    println!("{}", count);
}
