use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        c: [ Chars; h],
    }

    let mut ans = 0;
    for mask_i in 0..1 << h {
        for mask_j in 0..1 << w {
            let mut n = 0;
            for i in 0..h {
                for j in 0..w {
                    if (mask_i >> i) & 1 == 0 && (mask_j >> j) & 1 == 0 && c[i][j] == '#' {
                        n += 1;
                    }
                }
            }
            if n == k {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
