use proconio::marker::*;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [(isize, isize); n],
        m: usize,
    }

    type Mat = [[isize; 3]; 3];
    fn mul(a: &Mat, b: &Mat) -> Mat {
        let mut c = Mat::default();
        for (c, a) in c.iter_mut().zip(a.iter()) {
            for (a, b) in a.iter().zip(b.iter()) {
                for (c, b) in c.iter_mut().zip(b.iter()) {
                    *c += *a * *b;
                }
            }
        }
        c
    }

    let mut mat = Mat::default();
    for i in 0..3 {
        mat[i][i] = 1;
    }

    let mut memo = vec![];
    memo.push(mat);
    for _ in 0..m {
        input! {
            op: u8,
        }
        let trans = match op {
            1 => [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
            2 => [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
            3 => {
                input! {
                    p: isize,
                }
                [[-1, 0, 2 * p], [0, 1, 0], [0, 0, 1]]
            }
            _ => {
                input! {
                    p: isize,
                }
                [[1, 0, 0], [0, -1, 2 * p], [0, 0, 1]]
            }
        };

        mat = mul(&trans, &mat);
        memo.push(mat);
    }

    input! {
        q: usize,
        z: [(usize, Usize1); q],
    }

    for (a, b) in z {
        let (x, y) = p[b];
        let mat = memo[a];
        let res = mul(&mat, &[[x, 0, 0], [y, 0, 0], [1, 0, 0]]);
        let (x, y) = (res[0][0], res[1][0]);
        println!("{} {}", x, y);
    }
}
