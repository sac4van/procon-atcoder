use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        t: [[usize; n]; n],
    }

    let mut ans = 0;

    for p in (1..n).permutations(n - 1) {
        let mut d = t[0][p[0]];

        for i in 1..n - 1 {
            d += t[p[i - 1]][p[i]];
        }

        d += t[p[n - 2]][0];

        if d == k {
            ans += 1;
        }
    }

    println!("{}", ans);
}
