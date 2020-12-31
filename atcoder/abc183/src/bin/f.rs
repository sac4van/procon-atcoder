use proconio::{fastout, input};
use std::collections::HashMap;
use std::vec::Vec;

struct UnionFind {
    _parent: Vec<usize>,
    _size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            _parent: (0..n).collect(),
            _size: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self._parent[x] == x {
            x
        } else {
            self._parent[x] = self.root(self._parent[x]);
            self._parent[x]
        }
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut rx = self.root(x);
        let mut ry = self.root(y);
        if rx == ry {
            false
        } else {
            if self._size[rx] < self._size[ry] {
                let swp = rx;
                rx = ry;
                ry = swp;
            }
            self._parent[ry] = rx;
            self._size[rx] += self._size[ry];

            true
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn size(&mut self, x: usize) -> usize {
        let rx = self.root(x);
        self._size[rx]
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        c: [usize; n],
        x: [(usize, usize, usize); q],
    }

    let mut uf = UnionFind::new(n + 1);
    let mut class: Vec<HashMap<usize, isize>> = vec![HashMap::new(); n + 1];

    for &(a, x, y) in x.iter() {
        if a == 1 {
            let rx = uf.root(x);
            let ry = uf.root(y);
            if rx == ry {
                continue;
            }

            uf.unite(x, y);

            if rx != ry {
                    for (k, v) in class[ry].clone().iter() {
                        *class[rx].entry(*k).or_insert(0) += v;
                    }
                } else {
                    for (k, v) in class[rx].clone().iter() {
                        *class[ry].entry(*k).or_insert(0) += v;
                    }
                }
            }
        } else {
            let rx = uf.root(x);
            let ans = class[rx].entry(y).or_default();
            println!("{}", ans);
        }
    }
}
