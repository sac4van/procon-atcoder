use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        x: [(usize, usize, usize); n],
    }

    let mut demand = vec![0_i64; 200001];

    for &(s, t, p) in x.iter() {
        demand[s] += p as i64;
        demand[t] -= p as i64;
    }

    let mut cur = 0;
    let mut ok = true;
    for &d in demand.iter() {
        cur += d;
        if cur > w as i64 {
            ok = false;
            break;
        }
    }

    println!("{}", if ok { "Yes" } else { "No" });
}
