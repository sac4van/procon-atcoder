use proconio::marker::*;
use proconio::*;
use std::collections::*;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        c: (Usize1, Usize1),
        d: (Usize1, Usize1),
        s: [Chars; h],
    }

    let inf = 1 << 30;
    let mut dist = vec![vec![inf; w]; h];
    let mut used5x5 = vec![vec![false; w]; h];
    let mut que: VecDeque<(usize, usize)> = VecDeque::new();

    // list of "." (i,j) where dist[i][j] has determined for the first time.
    que.push_back(c);
    dist[c.0][c.1] = 0;

    let udlr = |x: (usize, usize)| -> Vec<(usize, usize)> {
        let mut y: Vec<(usize, usize)> = Vec::new();
        let (i, j) = x;
        for (di, dj) in vec![(1, 0), (0, 1), (2, 1), (1, 2)] {
            if i + di > 0 && j + dj > 0 && i + di < h + 1 && j + dj < w + 1 {
                let (k, l) = (i + di - 1, j + dj - 1);
                if s[k][l] == '.' {
                    y.push((k, l));
                }
            }
        }
        y
    };

    let warp = |x: (usize, usize)| -> Vec<(usize, usize)> {
        let mut y: Vec<(usize, usize)> = Vec::new();
        let (i, j) = x;
        for di in 0..5 {
            for dj in 0..5 {
                if i + di > 1 && j + dj > 1 && i + di < h + 2 && j + dj < w + 2 {
                    let (k, l) = (i + di - 2, j + dj - 2);
                    if s[k][l] == '.' && !(k == i && l == j) {
                        y.push((k, l));
                    }
                }
            }
        }
        y
    };

    while !que.is_empty() {
        let x = que.pop_front().unwrap();

        // search up-down-left-right
        let mut que2: VecDeque<(usize, usize)> = VecDeque::new();
        que2.push_back(x);

        while !que2.is_empty() {
            let (i, j) = que2.pop_front().unwrap();
            let d = dist[i][j];

            // search up-down-left-right
            for (k, l) in udlr((i, j)) {
                if s[k][l] == '.' && dist[k][l] != d {
                    dist[k][l] = d;
                    que2.push_back((k, l));
                }
            }
        }

        que2.push_back(x);
        while !que2.is_empty() {
            let (i, j) = que2.pop_front().unwrap();
            let d = dist[i][j];
            used5x5[i][j] = true;

            // search 5x5
            for (k, l) in warp((i, j)) {
                if dist[k][l] > d + 1 && s[k][l] == '.' {
                    dist[k][l] = d + 1;
                    que.push_back((k, l));
                }
            }

            for (k, l) in udlr((i, j)) {
                if s[k][l] == '.' && !used5x5[k][l] {
                    used5x5[k][l] = true;
                    que2.push_back((k, l));
                }
            }
        }
    }

    let ans = if dist[d.0][d.1] != inf {
        dist[d.0][d.1] as isize
    } else {
        -1
    };

    // debug
    // for i in 0..h {
    //     for j in 0..w {
    //         print!("{},", dist[i][j]);
    //     }
    //     println!("");
    // }

    println!("{}", ans);
}
