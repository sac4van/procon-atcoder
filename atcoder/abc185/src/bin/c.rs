use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l: u128,
    }

    let mut ans = 1;

    // ans:  combination(l-12+11, 11)
    for i in 1..=11 {
        ans *= l - i;
    }
    for i in 1..=11 {
        ans /= i;
    }

    println!("{}", ans);
}
