use proconio::marker::*;
use proconio::*;
use std::cmp::*;
use std::collections::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map: HashMap<(usize, usize, usize), f64> = HashMap::new();

    fn dfs(
        p: (usize, usize, usize, usize),
        map: &mut HashMap<(usize, usize, usize, usize), f64>,
    ) -> f64 {
        if !map.contains_key(&p) {
            let s = (p.0 + p.1 + p.2 + p.3) as f64;
            let ret = if p.0 > 0 {
                (1_f64 + dfs((p.0 - 1, p.1, p.2, p.3), map)) * (p.0 as f64) / s
            } else {
                0_f64
            } + if p.1 > 0 {
                (1_f64 + dfs((p.0, p.1 - 1, p.2, p.3), map)) * (p.1 as f64) / s
            } else {
                0_f64
            } + if p.2 > 0 {
                (1_f64 + dfs((p.0, p.1, p.2 - 1, p.3), map)) * (p.2 as f64) / s
            } else {
                0_f64
            } + if p.3 > 0 {
                (1_f64 + dfs((p.0, p.1, p.2, p.3 - 1), map)) * (p.3 as f64) / s
            } else {
                0_f64
            };
            map.entry(p).or_insert(ret);
        }
        *map.get(&p).unwrap()
    }

    let mut p = vec![0; 4];
    for ai in a {
        p[ai] += 1;
    }

    let ans = dfs((0));

    println!("{}", ans);
}
