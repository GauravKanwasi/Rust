use std::collections::{BTreeSet, HashMap};

fn find_kth_bit(n: i32, k: i32) -> char {
    debug_assert!(n >= 1, "n must be >= 1");
    debug_assert!(
        k >= 1 && (k as i64) < (1i64 << n),
        "k must be in 1..2^n - 1"
    );

    if n == 1 {
        return '0';
    }

    let mid = 1i64 << (n - 1);
    let k = k as i64;

    match k.cmp(&mid) {
        std::cmp::Ordering::Equal => '1',
        std::cmp::Ordering::Less => find_kth_bit(n - 1, k as i32),
        std::cmp::Ordering::Greater => {
            // Mirror k into the first half, then flip the bit we find there.
            let mirrored = (2 * mid - k) as i32;
            if find_kth_bit(n - 1, mirrored) == '0' {
                '1'
            } else {
                '0'
            }
        }
    }
}

fn avoid_flood(rains: Vec<i32>) -> Option<Vec<i32>> {
    let mut ans = vec![1i32; rains.len()];
    // Maps a currently-full lake to the day it was filled.
    let mut full_since: HashMap<i32, usize> = HashMap::new();
    // Indices of days with rains[i] == 0, still available to be used for draining.
    let mut dry_days: BTreeSet<usize> = BTreeSet::new();

    for (day, &lake) in rains.iter().enumerate() {
        if lake == 0 {
            dry_days.insert(day);
            continue;
        }

        ans[day] = -1; // Raining: this day cannot be used to drain a lake.

        if let Some(&filled_on) = full_since.get(&lake) {
            // This lake is already full — we must have drained it on some dry day
            // strictly between `filled_on` and today, otherwise it's a flood.
            let dry_day = dry_days.range(filled_on..day).next().copied()?;
            ans[dry_day] = lake;
            dry_days.remove(&dry_day);
        }

        full_since.insert(lake, day);
    }

    Some(ans)
}
fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
    let mut sorted_potions = potions;
    sorted_potions.sort_unstable();
    let total = sorted_potions.len() as i64;

    spells
        .iter()
        .map(|&spell| {
            // Smallest potion strength p such that spell * p >= success.
            let min_potion = success.div_ceil(spell as i64);
            let first_valid = sorted_potions.partition_point(|&p| (p as i64) < min_potion);
            (total - first_valid as i64) as i32
        })
        .collect()
}

fn main() {
    println!("=== Bit Navigator ===");
    println!("n=3, k=1  => {}", find_kth_bit(3, 1));
    println!("n=4, k=11 => {}", find_kth_bit(4, 11));

    println!("\n=== Flood Shield ===");
    for rains in [
        vec![1, 2, 3, 4],
        vec![1, 2, 0, 0, 2, 1],
        vec![1, 2, 0, 1, 2],
    ] {
        match avoid_flood(rains.clone()) {
            Some(plan) => println!("rains={:?} => {:?}", rains, plan),
            None => println!("rains={:?} => impossible (flood unavoidable)", rains),
        }
    }

    println!("\n=== Spell Matcher ===");
    println!(
        "spells=[5,1,3] potions=[1,2,3,4,5] success=7  => {:?}",
        successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7)
    );
    println!(
        "spells=[3,1,2] potions=[8,5,8] success=16     => {:?}",
        successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kth_bit_matches_known_values() {
        assert_eq!(find_kth_bit(1, 1), '0');
        assert_eq!(find_kth_bit(2, 1), '0');
        assert_eq!(find_kth_bit(2, 2), '1');
        assert_eq!(find_kth_bit(3, 1), '0');
        assert_eq!(find_kth_bit(4, 11), '1');
    }

    #[test]
    fn flood_shield_finds_valid_plan() {
        assert_eq!(avoid_flood(vec![1, 2, 0, 0, 2, 1]), Some(vec![-1, -1, 2, 1, -1, -1]));
    }

    #[test]
    fn flood_shield_detects_impossible_case() {
        // Lake 1 refills on day 2 with no dry day between days 0 and 2.
        assert_eq!(avoid_flood(vec![1, 0, 1]), None);
    }

    #[test]
    fn flood_shield_no_rain_at_all() {
        assert_eq!(avoid_flood(vec![0, 0, 0]), Some(vec![1, 1, 1]));
    }

    #[test]
    fn spell_matcher_basic_cases() {
        assert_eq!(
            successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7),
            vec![4, 0, 3]
        );
        assert_eq!(
            successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16),
            vec![2, 0, 2]
        );
    }
}
