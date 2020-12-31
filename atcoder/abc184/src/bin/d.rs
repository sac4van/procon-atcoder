use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: [usize; 3],
    }

    let mut e = vec![vec![vec![0_f64; 101]; 101]; 101];

    for i in (a[0]..100).rev() {
        for j in (a[1]..100).rev() {
            for k in (a[2]..100).rev() {
                let (_i, _j, _k) = (i as f64, j as f64, k as f64);
                e[i][j][k] = 1_f64
                    + (_i * e[i + 1][j][k] + _j * e[i][j + 1][k] + _k * e[i][j][k + 1])
                        / (_i + _j + _k);
            }
        }
    }

    println!("{}", e[a[0]][a[1]][a[2]]);
}
