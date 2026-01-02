use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let words = [
        "innovation",
        "resilience",
        "efficiency",
        "precision",
        "scalability",
        "integrity",
        "optimization",
        "automation",
        "security",
        "performance",
    ];

    let mut rng = thread_rng();
    let random_word = words.choose(&mut rng).unwrap();

    println!("Random word: {}", random_word);
}
