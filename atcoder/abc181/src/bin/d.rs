use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{fastout, input};
use std::cmp::min;
use std::vec::Vec;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut d_mod8 = vec![0; 8];

    for c in s {
        let _d = c as usize - '0' as usize;
        d_mod8[_d % 8] += 1;
    }

    let mut cand: Vec<usize> = Vec::new();
    for i in 0..8 {
        for _ in 0..min(3, d_mod8[i]) {
            cand.push(i);
        }
    }

    let ok = if cand.len() == 1 {
        cand[0] % 8 == 0
    } else if cand.len() == 2 {
        (2 * cand[1] + cand[0]) % 8 == 0 || (2 * cand[0] + cand[1]) % 8 == 0
    } else {
        let mut ret = false;
        for c in cand.iter().permutations(3) {
            if (4 * c[2] + 2 * c[1] + c[0]) % 8 == 0 {
                ret = true;
            }
        }
        ret
    };

    println!("{}", if ok { "Yes" } else { "No" });
}
