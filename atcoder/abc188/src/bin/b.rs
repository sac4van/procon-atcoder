use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [isize; n],
        b: [isize; n],
    }

    let mut p: isize = 0;
    for i in 0..n {
        p += a[i] * b[i];
    }

    let ans = if p == 0 { "Yes" } else { "No" };

    println!("{}", ans);
}
