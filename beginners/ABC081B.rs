use proconio::input;

fn main() {
    input! {
      n: usize,
      mut a: [usize; n]
    }

    let mut num = -1;
    let mut is_end = true;

    while is_end {
        for i in 0..n {
            if a[i] % 2 != 0 {
                is_end = false;
                break;
            } else {
                a[i] = a[i] / 2;
            }
        }
        num += 1;
    }

    println!("{}", num);
}
