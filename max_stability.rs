struct DSU {
    parent: Vec<usize>,
    rank: Vec<u8>,
    components: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            components: n,
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            let p = self.parent[x];
            self.parent[x] = self.parent[p];
            x = p;
        }
        x
    }

    fn unite(&mut self, mut a: usize, mut b: usize) -> bool {
        a = self.find(a);
        b = self.find(b);
        if a == b {
            return false;
        }
        if self.rank[a] < self.rank[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.parent[b] = a;
        if self.rank[a] == self.rank[b] {
            self.rank[a] += 1;
        }
        self.components -= 1;
        true
    }
}

impl Solution {
    fn can_achieve(n: usize, edges: &[(usize, usize, i32, i32)], k: i32, x: i32) -> bool {
        let mut dsu = DSU::new(n);
        let mut upgrades = 0;

        for &(u, v, s, must) in edges {
            if must == 1 {
                if s < x || !dsu.unite(u, v) {
                    return false;
                }
            }
        }

        for &(u, v, s, must) in edges {
            if must == 0 {
                if s >= x {
                    dsu.unite(u, v);
                } else if 2 * s >= x {
                    if dsu.unite(u, v) {
                        upgrades += 1;
                        if upgrades > k {
                            return false;
                        }
                    }
                }
            }
        }

        dsu.components == 1
    }

    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;

        let edges: Vec<(usize, usize, i32, i32)> = edges
            .into_iter()
            .map(|e| (e[0] as usize, e[1] as usize, e[2], e[3]))
            .collect();

        let mut dsu = DSU::new(n);
        for &(u, v, _, must) in &edges {
            if must == 1 && !dsu.unite(u, v) {
                return -1;
            }
        }

        let mut candidates = Vec::with_capacity(edges.len() * 2);
        for &(_, _, s, _) in &edges {
            candidates.push(s);
            candidates.push(s << 1);
        }
        candidates.sort_unstable();
        candidates.dedup();

        if !Self::can_achieve(n, &edges, k, candidates[0]) {
            return -1;
        }

        let mut lo = 0;
        let mut hi = candidates.len() - 1;
        let mut ans = candidates[0];

        while lo <= hi {
            let mid = (lo + hi) >> 1;
            if Self::can_achieve(n, &edges, k, candidates[mid]) {
                ans = candidates[mid];
                lo = mid + 1;
            } else {
                if mid == 0 { break; }
                hi = mid - 1;
            }
        }

        ans
    }
}
