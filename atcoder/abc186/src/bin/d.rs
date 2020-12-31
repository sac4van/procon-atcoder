use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
    }

    let mut ans: isize = 0;

    a.sort();
    for i in 0..n {
        let f: isize = 2 * (i as isize) + 1 - (n as isize);
        ans += a[i] * f;
    }

    println!("{}", ans);
}
