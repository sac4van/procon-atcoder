use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        v: [[usize; n]; k],
    }

    let mut ans = 0;
    let module = 1000000007;

    let mut dp = vec![vec![0 as usize; n], k];

    for i in 0..k {
        dp[n - 1][i] = 1;
    }

    for i in (0..(n - 1)).rev() {
        let mut p = k - 1;
        let mut q = k - 1;

        while p >= 0 && q >= 0 {
            if v[i][p] > v[i + 1][q] {
                q += 1;
            } else {
                dp[i][p] = k - q;
                p += 1;
            }
        }
    }

    println!("{}", ans);
}
