
use rand::distributions::WeightedIndex;
use rand::prelude::*;

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    Technical,
    Business,
    Security,
}

impl Category {
    const ALL: [Category; 3] = [
        Category::Technical,
        Category::Business,
        Category::Security,
    ];

    fn index(self) -> usize {
        match self {
            Category::Technical => 0,
            Category::Business => 1,
            Category::Security => 2,
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Category::Technical => "Technical",
                Category::Business => "Business",
                Category::Security => "Security",
            }
        )
    }
}

#[derive(Debug, Clone)]
struct Word {
    text: &'static str,
    category: Category,
    weight: u32,
}

struct WordSampler {
    words: Vec<Word>,
    history: Vec<usize>,
    counts: Vec<usize>,
    sampled: Vec<bool>,
    distribution: WeightedIndex<u32>,
}

impl WordSampler {
    fn new(words: Vec<Word>) -> Result<Self, String> {
        if words.is_empty() {
            return Err("Word list cannot be empty".into());
        }

        let weights: Vec<u32> = words.iter().map(|w| w.weight).collect();

        let distribution = WeightedIndex::new(&weights)
            .map_err(|e| format!("Invalid weights: {e}"))?;

        let n = words.len();

        Ok(Self {
            words,
            history: Vec::new(),
            counts: vec![0; n],
            sampled: vec![false; n],
            distribution,
        })
    }

    fn record(&mut self, index: usize) {
        self.history.push(index);
        self.counts[index] += 1;
        self.sampled[index] = true;
    }

    fn sample(&mut self, rng: &mut impl Rng) -> &Word {
        let index = self.distribution.sample(rng);
        self.record(index);
        &self.words[index]
    }

    fn sample_unique(&mut self, rng: &mut impl Rng) -> Option<&Word> {
        let weights: Vec<u32> = self.words
            .iter()
            .enumerate()
            .map(|(i, word)| {
                if self.sampled[i] {
                    0
                } else {
                    word.weight
                }
            })
            .collect();

        let distribution = WeightedIndex::new(&weights).ok()?;
        let index = distribution.sample(rng);

        self.record(index);

        Some(&self.words[index])
    }

    fn stats(&self) -> Vec<(&str, usize)> {
        self.words
            .iter()
            .zip(self.counts.iter())
            .filter(|(_, count)| **count > 0)
            .map(|(word, count)| (word.text, *count))
            .collect()
    }

    fn category_distribution(&self) -> [(Category, usize); 3] {
        let mut counts = [0usize; 3];

        for (word, &count) in self.words.iter().zip(&self.counts) {
            counts[word.category.index()] += count;
        }

        [
            (Category::Technical, counts[0]),
            (Category::Business, counts[1]),
            (Category::Security, counts[2]),
        ]
    }

    fn total_samples(&self) -> usize {
        self.history.len()
    }
}

fn build_words() -> Vec<Word> {
    vec![
        Word { text: "innovation", category: Category::Business, weight: 3 },
        Word { text: "resilience", category: Category::Business, weight: 2 },
        Word { text: "efficiency", category: Category::Business, weight: 3 },
        Word { text: "precision", category: Category::Technical, weight: 4 },
        Word { text: "scalability", category: Category::Technical, weight: 5 },
        Word { text: "integrity", category: Category::Security, weight: 2 },
        Word { text: "optimization", category: Category::Technical, weight: 4 },
        Word { text: "automation", category: Category::Technical, weight: 3 },
        Word { text: "security", category: Category::Security, weight: 5 },
        Word { text: "performance", category: Category::Technical, weight: 4 },
        Word { text: "observability", category: Category::Technical, weight: 2 },
        Word { text: "compliance", category: Category::Security, weight: 3 },
        Word { text: "throughput", category: Category::Technical, weight: 3 },
        Word { text: "governance", category: Category::Business, weight: 2 },
        Word { text: "reliability", category: Category::Technical, weight: 4 },
    ]
}

fn main() {
    let mut rng = thread_rng();

    let mut sampler = match WordSampler::new(build_words()) {
        Ok(sampler) => sampler,
        Err(error) => {
            eprintln!("{error}");
            return;
        }
    };

    println!("\n========== WEIGHTED RANDOM SAMPLES ==========");

    for _ in 0..5 {
        let word = sampler.sample(&mut rng);
        println!("  {:<12} {}", word.category, word.text);
    }

    println!("\n========== UNIQUE SAMPLES ==========");

    for _ in 0..5 {
        match sampler.sample_unique(&mut rng) {
            Some(word) => {
                println!("  {:<12} {}", word.category, word.text);
            }
            None => {
                println!("  No more weighted samples available");
                break;
            }
        }
    }

    println!("\n========== WORD FREQUENCY ==========");

    let mut stats = sampler.stats();
    stats.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));

    for (word, count) in stats {
        println!("  {:<16} {:<20} {}", word, "#".repeat(count), count);
    }

    println!("\n========== CATEGORY DISTRIBUTION ==========");

    let categories = sampler.category_distribution();
    let total = sampler.total_samples();

    for (category, count) in categories {
        let percentage = if total > 0 {
            count as f64 / total as f64 * 100.0
        } else {
            0.0
        };

        println!(
            "  {:<12} {:>3} samples  ({:>5.1}%)",
            category, count, percentage
        );
    }

    println!("\n============================================");
    println!("  Total Samples: {}", total);
    println!("============================================");
}
