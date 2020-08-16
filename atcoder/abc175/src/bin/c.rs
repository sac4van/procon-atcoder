use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: i128,
        k: i128,
        d: i128,
    }

    let ans;

    if x.abs() > k * d {
        ans = x.abs() - k * d;
    } else {
        let q = x.abs() / d;
        let r = x.abs() % d;
        if (k - q) & 1 == 0 {
            ans = r;
        } else {
            ans = d - r;
        }
    }

    println!("{}", ans);
}
