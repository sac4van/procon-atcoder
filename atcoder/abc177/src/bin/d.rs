use proconio::*;
use std::cmp::*;

#[allow(dead_code)]
struct UnionFind {
    size: usize,
    pos: Vec<isize>,
}

#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> Self {
        let size = n;
        let pos = vec![-1; n + 1];
        UnionFind { size, pos }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.pos[x] < 0 {
            x
        } else {
            let v = self.pos[x] as usize;
            self.pos[x] = self.find(v) as isize;
            self.pos[x] as usize
        }
    }
    fn unite(&mut self, x: usize, y: usize) -> Result<(), ()> {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x == y {
            return Err(());
        };
        if self.pos[x] > self.pos[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.pos[x] += self.pos[y];
        self.pos[y] = x as isize;
        Ok(())
    }
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
    fn size(&mut self, x: usize) -> usize {
        let x = self.find(x);
        -self.pos[x] as usize
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        rel: [(usize, usize); m],
    }

    let mut uf = UnionFind::new(n);
    for (a, b) in rel {
        uf.unite(a, b).ok();
    }

    let mut ans = 0;
    for i in 1..=n {
        let s = uf.size(i);
        ans = max(ans, s);
    }

    println!("{}", ans);
}
