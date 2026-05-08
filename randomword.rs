use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
struct Word {
    text: &'static str,
    category: Category,
    weight: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Category {
    Technical,
    Business,
    Security,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Technical => write!(f, "Technical"),
            Category::Business => write!(f, "Business"),
            Category::Security => write!(f, "Security"),
        }
    }
}

struct WordSampler {
    words: Vec<Word>,
    history: Vec<String>,
    dist: WeightedIndex<u32>,
}

impl WordSampler {
    fn new(words: Vec<Word>) -> Self {
        let weights: Vec<u32> = words.iter().map(|w| w.weight).collect();
        let dist = WeightedIndex::new(&weights).expect("weights must be valid");
        Self { words, history: Vec::new(), dist }
    }

    fn sample(&mut self, rng: &mut impl Rng) -> &Word {
        let idx = self.dist.sample(rng);
        let word = &self.words[idx];
        self.history.push(word.text.to_string());
        word
    }

    fn sample_unique(&mut self, rng: &mut impl Rng) -> Option<&Word> {
        let seen: std::collections::HashSet<&str> = self.history.iter().map(|s| s.as_str()).collect();
        let available: Vec<usize> = self.words.iter().enumerate()
            .filter(|(_, w)| !seen.contains(w.text))
            .map(|(i, _)| i)
            .collect();
        if available.is_empty() {
            return None;
        }
        let idx = *available.choose(rng).unwrap();
        let word = &self.words[idx];
        self.history.push(word.text.to_string());
        Some(word)
    }

    fn stats(&self) -> HashMap<String, usize> {
        let mut counts: HashMap<String, usize> = HashMap::new();
        for word in &self.history {
            *counts.entry(word.clone()).or_insert(0) += 1;
        }
        counts
    }

    fn category_distribution(&self) -> HashMap<String, usize> {
        let mut dist: HashMap<String, usize> = HashMap::new();
        for word_text in &self.history {
            if let Some(word) = self.words.iter().find(|w| w.text == word_text) {
                *dist.entry(word.category.to_string()).or_insert(0) += 1;
            }
        }
        dist
    }
}

fn build_words() -> Vec<Word> {
    vec![
        Word { text: "innovation",    category: Category::Business,  weight: 3 },
        Word { text: "resilience",    category: Category::Business,  weight: 2 },
        Word { text: "efficiency",    category: Category::Business,  weight: 3 },
        Word { text: "precision",     category: Category::Technical, weight: 4 },
        Word { text: "scalability",   category: Category::Technical, weight: 5 },
        Word { text: "integrity",     category: Category::Security,  weight: 2 },
        Word { text: "optimization",  category: Category::Technical, weight: 4 },
        Word { text: "automation",    category: Category::Technical, weight: 3 },
        Word { text: "security",      category: Category::Security,  weight: 5 },
        Word { text: "performance",   category: Category::Technical, weight: 4 },
        Word { text: "observability", category: Category::Technical, weight: 2 },
        Word { text: "compliance",    category: Category::Security,  weight: 3 },
        Word { text: "throughput",    category: Category::Technical, weight: 3 },
        Word { text: "governance",    category: Category::Business,  weight: 2 },
        Word { text: "reliability",   category: Category::Technical, weight: 4 },
    ]
}

fn main() {
    let mut rng = thread_rng();
    let mut sampler = WordSampler::new(build_words());

    println!("=== Weighted Random Samples ===");
    for _ in 0..5 {
        let word = sampler.sample(&mut rng);
        println!("  [{:<10}] {}", word.category.to_string(), word.text);
    }

    println!("\n=== Unique Samples (no repeats) ===");
    for _ in 0..5 {
        match sampler.sample_unique(&mut rng) {
            Some(word) => println!("  [{:<10}] {}", word.category.to_string(), word.text),
            None => println!("  (all words exhausted)"),
        }
    }

    println!("\n=== Word Frequency ===");
    let mut stats: Vec<(String, usize)> = sampler.stats().into_iter().collect();
    stats.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    for (word, count) in &stats {
        let bar = "#".repeat(*count);
        println!("  {:<15} {} ({})", word, bar, count);
    }

    println!("\n=== Category Distribution ===");
    let mut cat_dist: Vec<(String, usize)> = sampler.category_distribution().into_iter().collect();
    cat_dist.sort_by(|a, b| b.1.cmp(&a.1));
    let total: usize = cat_dist.iter().map(|(_, c)| c).sum();
    for (cat, count) in &cat_dist {
        let pct = (*count as f64 / total as f64) * 100.0;
        println!("  {:<12} {:>2} samples  ({:.0}%)", cat, count, pct);
    }
}
