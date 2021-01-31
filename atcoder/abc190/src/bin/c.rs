use proconio::marker::*;
use proconio::*;
use std::cmp::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
        k: usize,
        cd: [(Usize1, Usize1); k],
    }

    let mut ans = 0;
    for cfg in 0..(1 << k) {
        let mut ball = vec![false; n];
        let mut c = cfg;
        for i in 0..k {
            let selected = if c & 1 == 0 { cd[i].0 } else { cd[i].1 };
            ball[selected] = true;
            c >>= 1;
        }

        let mut tmp = 0;

        for &(a, b) in &ab {
            if ball[a] && ball[b] {
                tmp += 1;
            }
        }

        ans = ans.max(tmp);
    }

    println!("{}", ans);
}
