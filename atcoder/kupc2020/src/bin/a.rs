use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: [isize; 2*n]
    }

    fn abs(a: isize) -> isize {
        if a > 0 {
            a
        } else {
            -a
        }
    }

    let mut ans = 0;

    for i in 1..n {
        ans += abs(x[2 * i - 2] - x[2 * i]) + abs(x[2 * i - 1] - x[2 * i + 1]);
    }

    println!("{}", ans);
}
