struct DSU {
    parent: Vec<usize>,
    rank: Vec<usize>,
    components: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            parent: (0..n).collect(),
            rank: vec![0; n],
            components: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn unite(&mut self, a: usize, b: usize) -> bool {
        let a = self.find(a);
        let b = self.find(b);
        if a == b {
            return false;
        }
        if self.rank[a] < self.rank[b] {
            self.parent[a] = b;
        } else if self.rank[a] > self.rank[b] {
            self.parent[b] = a;
        } else {
            self.parent[b] = a;
            self.rank[a] += 1;
        }
        self.components -= 1;
        true
    }
}

impl Solution {
    fn can_achieve(n: usize, edges: &Vec<Vec<i32>>, k: i32, x: i32) -> bool {
        let mut dsu = DSU::new(n);

        for e in edges {
            let (u, v, s, must) = (e[0] as usize, e[1] as usize, e[2], e[3]);
            if must == 1 {
                if s < x {
                    return false;
                }
                if !dsu.unite(u, v) {
                    return false;
                }
            }
        }

        for e in edges {
            let (u, v, s, must) = (e[0] as usize, e[1] as usize, e[2], e[3]);
            if must == 0 && s >= x {
                dsu.unite(u, v);
            }
        }

        let mut used_upgrades = 0i32;
        for e in edges {
            let (u, v, s, must) = (e[0] as usize, e[1] as usize, e[2], e[3]);
            if must == 0 && s < x && 2 * s >= x {
                if dsu.unite(u, v) {
                    used_upgrades += 1;
                    if used_upgrades > k {
                        return false;
                    }
                }
            }
        }

        dsu.components == 1
    }

    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;

        let mut dsu = DSU::new(n);
        for e in &edges {
            if e[3] == 1 && !dsu.unite(e[0] as usize, e[1] as usize) {
                return -1;
            }
        }

        let mut candidates: Vec<i32> = Vec::new();
        for e in &edges {
            candidates.push(e[2]);
            candidates.push(2 * e[2]);
        }
        candidates.sort_unstable();
        candidates.dedup();

        if !Self::can_achieve(n, &edges, k, candidates[0]) {
            return -1;
        }

        let mut lo = 0usize;
        let mut hi = candidates.len();
        let mut ans = candidates[0];

        while lo < hi {
            let mid = (lo + hi) / 2;
            if Self::can_achieve(n, &edges, k, candidates[mid]) {
                ans = candidates[mid];
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        ans
    }
}
