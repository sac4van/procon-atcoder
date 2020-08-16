use proconio::{fastout, input};
use std::cmp;

fn accumurate(l: Vec<i64>) -> Vec<i64> {
    let n = l.len();
    let mut ret = vec![0; n];
    ret[0] = l[0];
    for i in 1..n {
        ret[i] = ret[i - 1] + l[i];
    }

    return ret;
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
        c: [i64; n],
    }
    let inf = 1 << 32;
    let mut ans = -inf;
    let mut used = vec![false; n];

    for i in 0..n {
        if used[i] {
            continue;
        } else {
            used[i] = true;
        }

        let mut c_loop: Vec<i64> = Vec::new();
        let mut j = p[i] - 1;
        c_loop.push(c[j]);

        while j != i {
            used[j] = true;
            j = p[j] - 1;
            c_loop.push(c[j]);
        }

        let n_loop = c_loop.len();
        c_loop.append(&mut c_loop.to_vec());
        let cs_loop = accumurate(c_loop);
        let tot_loop = cs_loop[n_loop - 1];

        let _jmax = cmp::min(k, n_loop);

        for _i in 0..n_loop {
            for _j in 1..(_jmax + 1) {
                let mut _tmp = cs_loop[_i + _j] - cs_loop[_i];

                if tot_loop > 0 && k > _j {
                    _tmp += (((k - _j) / n_loop) as i64) * tot_loop;
                }

                if ans < _tmp {
                    ans = _tmp;
                }
            }
        }
    }

    print!("{}", ans);
}
