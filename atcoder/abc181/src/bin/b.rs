use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(u128, u128); n],
    }

    let mut ans = 0_u128;

    for (a, b) in ab {
        ans += b * (b + 1) / 2 - (a - 1) * a / 2;
    }

    println!("{}", ans);
}
