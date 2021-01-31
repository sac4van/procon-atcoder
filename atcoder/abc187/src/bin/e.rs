use proconio::marker::*;
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        e: [(Usize1, Usize1); n-1],
        q: usize,
        ask: [(u8, Usize1, isize); q]
    }

    // graph
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    for &(a, b) in &e {
        g[a].push(b);
        g[b].push(a);
    }
    let mut parent: Vec<usize> = vec![0; n];
    let mut children: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut xs: Vec<isize> = vec![0; n];

    // tree (root as 0th node)
    let mut used: Vec<bool> = vec![false; n];
    let mut que: Vec<usize> = Vec::new();
    que.push(0);
    used[0] = true;
    while !que.is_empty() {
        let current = que.pop().unwrap();
        for &neighbor in &g[current] {
            if !used[neighbor] {
                used[neighbor] = true;
                que.push(neighbor);
                parent[neighbor] = current;
                children[current].push(neighbor);
            }
        }
    }

    let mut sum2 = 0;
    for (ti, ei, xi) in ask {
        let (mut a, mut b) = &e[ei];
        if ti == 2 {
            std::mem::swap(&mut a, &mut b);
        }

        if parent[b] == a {
            xs[b] -= xi;
            sum2 += xi;
        } else {
            xs[a] += xi;
        }
    }

    let mut c: Vec<isize> = vec![0; n];

    que.clear();
    que.push(0);

    // breadth-first update
    while !que.is_empty() {
        let current = que.pop().unwrap();
        c[current] += xs[current];
        for &child in &children[current] {
            c[child] += c[current];
            que.push(child);
        }
    }

    for ci in c {
        let ans = ci + sum2;
        println!("{}", ans);
    }
}
