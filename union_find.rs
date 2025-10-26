
use std::mem::swap;
struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            rank: vec![0; n],
            siz: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            self.par[x] = self.root(self.par[x]);
            self.par[x]
        }
    }

    fn isssame(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut rx = self.root(x);
        let mut ry = self.root(y);
        if rx == ry {
            return false;
        }

        //union by rank

        //ry側のrankが小さくなるようにする
        if self.rank[rx] < self.rank[ry] {
            swap(&mut rx, &mut ry);
        }
        self.par[ry] = rx; //ryをrxの子とする

        if self.rank[rx] == self.rank[ry] {
            self.rank[rx] += 1;
        }
        self.siz[rx] += self.siz[ry];

        return true;
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}
