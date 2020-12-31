use proconio::marker::Usize1;
use proconio::{fastout, input};
use std::vec::Vec;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        edge: [(Usize1, Usize1, usize); m]
    }

    let mut graph: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];

    for &e in edge.iter() {
        graph[e.0].push((e.1, e.2));
        graph[e.1].push((e.0, e.2));
    }

    let mut ans = vec![0; n];
    ans[0] = 1;

    let mut q = std::collections::VecDeque::<usize>::new();
    q.push_back(0);

    while let Some(v) = q.pop_front() {
        for &(u, c) in graph[v].iter() {
            if ans[u] != 0 {
                continue;
            }

            ans[u] = if ans[v] == c {
                if c != 1 {
                    1
                } else {
                    2
                }
            } else {
                c
            };
            q.push_back(u);
        }
    }

    for a in ans.iter() {
        println!("{}", a);
    }
}
