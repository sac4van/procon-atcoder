use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    // greatest coordinate, using a_1, ..., a_i
    let mut max_d = Vec::new();
    // accumulation of a
    let mut acc_a = Vec::new();
    acc_a.push(0);
    let mut max_d_i = 0_i64;
    let mut _d = 0_i64;
    for _a in a {
        _d += _a;
        if _d > max_d_i {
            max_d_i = _d;
        }
        max_d.push(max_d_i);
        acc_a.push(_d);
    }
    let mut ans = 0;
    _d = 0;
    for i in 0..n {
        let d1 = _d; // previous max
        let d2 = _d + max_d[i]; // next
        _d += acc_a[i + 1];
        let d3 = _d;
        let cand = vec![ans, d1, d2, d3];
        ans = *cand.iter().max().unwrap();
    }

    println!("{}", ans);
}
