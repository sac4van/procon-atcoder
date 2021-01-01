use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut ans = 0;

    for a in 1..n {
        ans += (n - 1) / a;
    }

    println!("{}", ans);
}
