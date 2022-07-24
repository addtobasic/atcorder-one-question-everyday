use proconio::input;

fn main() {
    input! {
        n: usize,
        v : [[i32; 3]; n]
    }

    let mut pt = 0;
    let mut px = 0;
    let mut py = 0;

    for i in 0..(n as usize) {
        let d = (px - v[i][1]).abs() + (py - v[i][2]).abs();
        let dt =  v[i][0] - pt;

        if dt < d || (dt - d) % 2 == 1 {
            println!("No");
            return;
        }

        pt = v[i][0];
        px = v[i][1];
        py = v[i][2];
    }

    println!("Yes");
}
