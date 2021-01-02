use proconio::marker::*;
use proconio::*;
use std::collections::*;

mod union_find {
    #[allow(dead_code)]
    pub struct DSU {
        size: usize,
        pos: Vec<isize>,
    }
    #[allow(dead_code)]
    impl DSU {
        pub fn new(n: usize) -> Self {
            let size = n;
            let pos = vec![-1; n];
            DSU { size, pos }
        }
        pub fn find(&mut self, x: usize) -> usize {
            if self.pos[x] < 0 {
                x
            } else {
                let v = self.pos[x] as usize;
                self.pos[x] = self.find(v) as isize;
                self.pos[x] as usize
            }
        }
        pub fn unite(&mut self, x: usize, y: usize) -> Result<usize, ()> {
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
            Ok(x)
        }
        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.find(x) == self.find(y)
        }
        pub fn size(&mut self, x: usize) -> usize {
            let x = self.find(x);
            -self.pos[x] as usize
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        c: [Usize1; n],
        ask: [(usize, Usize1, Usize1); q],
    }

    let mut uf = union_find::DSU::new(n);
    let mut class: Vec<HashMap<usize, usize>> = vec![HashMap::new(); n];

    for (i, ci) in c.iter().enumerate() {
        class[i].insert(*ci, 1);
    }

    for &(a, x, y) in ask.iter() {
        if a == 1 {
            let rx = uf.find(x);
            let ry = uf.find(y);
            if rx != ry {
                let nx = uf.unite(x, y).unwrap();
                let (from, to) = if rx == nx { (ry, rx) } else { (rx, ry) };

                for (k, v) in class[from].clone().iter() {
                    *class[to].entry(*k).or_insert(0) += v;
                }
            }
        } else {
            let rx = uf.find(x);
            let ans = class[rx].entry(y).or_default();
            println!("{}", ans);
        }
    }
}
