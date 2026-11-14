use rand::prelude::*;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Category {
    Technical,
    Business,
    Security,
}

struct Word {
    text: &'static str,
    category: Category,
    weight: u32,
}

struct Sampler {
    words: Vec<Word>,
    counts: Vec<usize>,
}

impl Sampler {
    fn new(words: Vec<Word>) -> Self {
        let counts = vec![0; words.len()];
        Self { words, counts }
    }

    fn pick(&mut self, rng: &mut impl Rng, unique: bool) -> Option<&Word> {
        let candidates: Vec<usize> = (0..self.words.len())
            .filter(|&i| !unique || self.counts[i] == 0)
            .collect();

        let &i = candidates
            .choose_weighted(rng, |&i| self.words[i].weight)
            .ok()?;

        self.counts[i] += 1;
        Some(&self.words[i])
    }

    fn total(&self) -> usize {
        self.counts.iter().sum()
    }

    fn frequencies(&self) -> Vec<(&'static str, usize)> {
        let mut freq: Vec<_> = self
            .words
            .iter()
            .zip(&self.counts)
            .filter(|(_, &n)| n > 0)
            .map(|(w, &n)| (w.text, n))
            .collect();
        freq.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
        freq
    }

    fn by_category(&self) -> BTreeMap<Category, usize> {
        let mut map = BTreeMap::new();
        for (w, &n) in self.words.iter().zip(&self.counts) {
            *map.entry(w.category).or_insert(0) += n;
        }
        map
    }
}

fn words() -> Vec<Word> {
    use Category::*;
    [
        ("innovation", Business, 3),
        ("resilience", Business, 2),
        ("efficiency", Business, 3),
        ("precision", Technical, 4),
        ("scalability", Technical, 5),
        ("integrity", Security, 2),
        ("optimization", Technical, 4),
        ("automation", Technical, 3),
        ("security", Security, 5),
        ("performance", Technical, 4),
        ("observability", Technical, 2),
        ("compliance", Security, 3),
        ("throughput", Technical, 3),
        ("governance", Business, 2),
        ("reliability", Technical, 4),
    ]
    .into_iter()
    .map(|(text, category, weight)| Word { text, category, weight })
    .collect()
}

fn main() {
    let mut rng = thread_rng();
    let mut sampler = Sampler::new(words());

    println!("\n=== WEIGHTED SAMPLES ===");
    for _ in 0..5 {
        if let Some(w) = sampler.pick(&mut rng, false) {
            println!("  {:<12} {}", format!("{:?}", w.category), w.text);
        }
    }

    println!("\n=== UNIQUE SAMPLES ===");
    for _ in 0..5 {
        match sampler.pick(&mut rng, true) {
            Some(w) => println!("  {:<12} {}", format!("{:?}", w.category), w.text),
            None => {
                println!("  No more unique words");
                break;
            }
        }
    }

    println!("\n=== FREQUENCY ===");
    for (text, n) in sampler.frequencies() {
        println!("  {:<16} {:<10} {}", text, "#".repeat(n), n);
    }

    let total = sampler.total();
    println!("\n=== CATEGORIES ===");
    for (category, n) in sampler.by_category() {
        let pct = if total > 0 { n as f64 * 100.0 / total as f64 } else { 0.0 };
        println!("  {:<12} {:>3}  ({:>5.1}%)", format!("{:?}", category), n, pct);
    }

    println!("\nTotal samples: {total}");
}
