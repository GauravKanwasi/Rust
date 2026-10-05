#[derive(Clone)]
struct Dsu {
    parent: Vec<usize>,
    rank: Vec<u8>,
    components: usize,
}

impl Dsu {
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

    fn unite(&mut self, a: usize, b: usize) -> bool {
        let (mut a, mut b) = (self.find(a), self.find(b));
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
    fn feasible(base: &Dsu, optional: &[(usize, usize, i32)], k: i32, x: i32) -> bool {
        let mut dsu = base.clone();

        for &(u, v, s) in optional {
            if s >= x {
                dsu.unite(u, v);
            }
        }

        let mut upgrades = 0;
        for &(u, v, s) in optional {
            if s < x && s * 2 >= x && dsu.unite(u, v) {
                upgrades += 1;
                if upgrades > k {
                    return false;
                }
            }
        }

        dsu.components == 1
    }

    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;
        let mut base = Dsu::new(n);
        let mut optional = Vec::with_capacity(edges.len());
        let mut min_must = i32::MAX;
        let mut max_strength = 0;

        for e in &edges {
            let (u, v, s) = (e[0] as usize, e[1] as usize, e[2]);
            max_strength = max_strength.max(s);
            if e[3] == 1 {
                if !base.unite(u, v) {
                    return -1;
                }
                min_must = min_must.min(s);
            } else {
                optional.push((u, v, s));
            }
        }

        let mut lo = 1;
        let mut hi = min_must.min(max_strength * 2);
        let mut ans = -1;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if Self::feasible(&base, &optional, k, mid) {
                ans = mid;
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }

        ans
    }
}
