use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        c: isize,
        b: [isize; m],
        a: [[isize; m]; n]
    }

    let mut ans = 0;

    for i in 0..n {
        let mut score = c;
        for j in 0..m {
            score += a[i][j] * b[j];
        }

        if score > 0 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
