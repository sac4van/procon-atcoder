use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut count: Vec<usize> = vec![0; n];

    for ai in a {
        count[ai] += 1;
    }

    let mut ans = 0;

    let mut w = k;
    for i in 0..n {
        if count[i] < w {
            ans += (w - count[i]) * i;
            w = count[i];
        }
    }

    println!("{}", ans);
}
