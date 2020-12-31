use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::{HashMap, VecDeque};
use std::vec::Vec;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }

    let mut char_to_ijs: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for i in 0..h {
        for j in 0..w {
            let _a = a[i][j];
            if !(_a == '.' || _a == '#') {
                char_to_ijs.entry(_a).or_default().push((i, j));
            }
        }
    }
    let unused = 1 << 30;
    let mut dist = vec![vec![unused; w]; h];
    let mut next_ijs: VecDeque<(usize, usize)> = VecDeque::new();

    let s = char_to_ijs.get(&'S').unwrap()[0];
    let g = char_to_ijs.get(&'G').unwrap()[0];

    dist[s.0][s.1] = 0;
    next_ijs.push_back(s);

    let _next = |p: (usize, usize)| -> Vec<(usize, usize)> {
        let mut ret: Vec<(usize, usize)> = Vec::new();
        if p.0 > 0 {
            ret.push((p.0 - 1, p.1));
        }

        if p.0 < h - 1 {
            ret.push((p.0 + 1, p.1));
        }

        if p.1 > 0 {
            ret.push((p.0, p.1 - 1));
        }

        if p.1 < w - 1 {
            ret.push((p.0, p.1 + 1));
        }

        ret
    };

    while !next_ijs.is_empty() {
        let p = next_ijs.pop_front().unwrap();
        let dist_p = dist[p.0][p.1];

        if p == g {
            break;
        }

        // up-down, left-right neighbors
        for q in _next(p) {
            let char_q = a[q.0][q.1];

            if char_q == '#' {
                continue;
            }

            // teleport
            if dist[q.0][q.1] == unused && char_to_ijs.contains_key(&char_q) {
                for &r in char_to_ijs.get(&char_q).unwrap() {
                    if r != q && dist[r.0][r.1] > dist_p + 2 {
                        dist[r.0][r.1] = dist_p + 2;
                        next_ijs.push_back(r);
                    }
                }
            }

            if dist[q.0][q.1] > dist_p + 1 {
                dist[q.0][q.1] = dist_p + 1;
                next_ijs.push_back(q);
            }
        }
    }

    println!(
        "{}",
        if dist[g.0][g.1] != unused {
            dist[g.0][g.1]
        } else {
            -1
        }
    );
}
