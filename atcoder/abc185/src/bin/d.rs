use proconio::{fastout, input};
use std::cmp::min;
use std::vec::Vec;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; m],
    }

    a.sort();

    let ans = if m == 0 {
        1
    } else {
        let mut k = n;
        let mut d: Vec<usize> = Vec::new();

        if a[0] != 1 {
            k = a[0] - 1;
            d.push(a[0] - 1);
        }
        for i in 1..m {
            if a[i] - a[i - 1] > 1 {
                k = min(k, a[i] - a[i - 1] - 1);
                d.push(a[i] - a[i - 1] - 1);
            }
        }
        if a[m - 1] != n {
            k = min(k, n - a[m - 1]);
            d.push(n - a[m - 1]);
        }

        let mut _ans = 0;
        for _d in d {
            _ans += (_d + k - 1) / k;
        }
        _ans
    };
    println!("{}", ans);
}
