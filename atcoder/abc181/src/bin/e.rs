use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut h: [usize; n],
        w: [usize; m]
    }

    h.sort();

    let mut acc_eo = vec![0; 1 + n / 2];
    let mut acc_oe = vec![0; 1 + n / 2];

    for i in 0..(n / 2) {
        acc_eo[i + 1] = acc_eo[i] + h[2 * i + 1] - h[2 * i];
        acc_oe[i + 1] = acc_oe[i] + h[2 * i + 2] - h[2 * i + 1];
    }

    let mut min_sum = 1 << 30;

    for wj in w {
        let pos = match h.binary_search(&wj) {
            Ok(id) => id,
            Err(id) => id,
        };

        let sum = acc_eo[pos / 2]
            + (if pos & 1 == 0 {
                h[pos] - wj
            } else {
                wj - h[pos - 1]
            })
            + acc_oe[n / 2]
            - acc_oe[pos / 2];

        min_sum = min(min_sum, sum);
    }

    println!("{}", min_sum);
}
