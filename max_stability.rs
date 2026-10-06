#[derive(Clone)]
struct Dsu {
    parent: Vec<usize>,
    size: Vec<u32>,
    components: usize,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            components: n,
        }
    }

    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    fn unite(&mut self, a: usize, b: usize) -> bool {
        let (mut a, mut b) = (self.find(a), self.find(b));
        if a == b {
            return false;
        }
        if self.size[a] < self.size[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.parent[b] = a;
        self.size[a] += self.size[b];
        self.components -= 1;
        true
    }
}

impl Solution {
    fn feasible(
        base: &Dsu,
        scratch: &mut Dsu,
        optional: &[(usize, usize, i32)],
        k: i32,
        x: i32,
    ) -> bool {
        scratch.clone_from(base);

        let free = optional.partition_point(|e| e.2 >= x);
        for &(u, v, _) in &optional[..free] {
            scratch.unite(u, v);
        }

        let mut left = k;
        for &(u, v, s) in &optional[free..] {
            if s * 2 < x {
                break;
            }
            if scratch.unite(u, v) {
                left -= 1;
                if left < 0 {
                    return false;
                }
            }
        }

        scratch.components == 1
    }

    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;
        let mut base = Dsu::new(n);
        let mut optional: Vec<(usize, usize, i32)> = Vec::with_capacity(edges.len());
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

        if base.components - 1 > optional.len() {
            return -1;
        }

        optional.sort_unstable_by(|a, b| b.2.cmp(&a.2));

        let mut scratch = base.clone();
        let (mut lo, mut hi) = (1, min_must.min(max_strength * 2));
        let mut ans = -1;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if Self::feasible(&base, &mut scratch, &optional, k, mid) {
                ans = mid;
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }

        ans
    }
}
