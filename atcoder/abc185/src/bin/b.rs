use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        n: isize,
        m: usize,
        t: isize,
        ab: [(isize, isize); m],
    }

    let mut ok = true;
    let mut cur_time = 0_isize;
    let mut cur_battery = n;

    for (a, b) in ab {
        cur_battery -= a - cur_time;
        if cur_battery <= 0 {
            ok = false;
            break;
        }
        cur_battery = min(n, cur_battery + b - a);
        cur_time = b;
    }

    cur_battery -= t - cur_time;

    if cur_battery <= 0 {
        ok = false;
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
