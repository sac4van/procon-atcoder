use proconio::marker::*;
use proconio::*;
use std::collections::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize; n],
        xy: [(Usize1, Usize1); m],
    }
    let mut p: Vec<isize> = vec![0; n];
    let mut g: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for (x, y) in xy {
        g[y].insert(x);
    }

    fn dfs(x: usize, p: &mut Vec<isize>, a: &Vec<isize>, g: &Vec<HashSet<usize>>) -> isize {
        if p[x] == 0 {
            p[x] = a[x];
            if g[x].len() != 0 {
                for &y in &g[x] {
                    let py = dfs(y, p, a, g);
                    p[x] = p[x].min(py);
                }
            }
        }
        p[x]
    };

    let mut ans: isize = -(1 << 30);
    for i in 0..n {
        if g[i].len() == 0 {
            continue;
        }

        for &j in &g[i] {
            ans = ans.max(a[i] - dfs(j, &mut p, &a, &g));
        }
    }

    println!("{}", ans);
}
