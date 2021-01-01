use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        x: u128,
        y: u128,
        a: u128,
        b: u128,
    }

    let mut v_str = x;
    let mut v_exp = 0_u128;

    loop {
        if v_str * a >= min(b / (a - 1), y - 1) {
            break;
        }

        v_str *= a;
        v_exp += 1;
    }

    v_exp += (y - 1 - v_str) / b;

    println!("{}", v_exp);
}
