use std::collections::{BTreeSet, HashMap};

// Bit Navigator
fn find_kth_bit(n: i32, k: i32) -> char {
    if n == 1 { return '0'; }
    let mid = (1 << n) / 2;
    if k == mid { '1' }
    else if k < mid { find_kth_bit(n - 1, k) }
    else {
        let mirrored = mid * 2 - k;
        if find_kth_bit(n - 1, mirrored) == '0' { '1' } else { '0' }
    }
}

// Flood Shield
fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![1i32; rains.len()];
    let mut full: HashMap<i32, usize> = HashMap::new();
    let mut dry_days: BTreeSet<usize> = BTreeSet::new();
    let mut pending: Vec<(usize, i32)> = Vec::new();

    for (i, &rain) in rains.iter().enumerate() {
        if rain == 0 { dry_days.insert(i); }
        else {
            if let Some(&last) = full.get(&rain) { pending.push((last, rain)); }
            full.insert(rain, i);
            ans[i] = -1;
        }
    }

    for (last, lake) in pending {
        if let Some(&dry_day) = dry_days.range(last..).next() {
            ans[dry_day] = lake;
            dry_days.remove(&dry_day);
        } else { return vec![]; }
    }
    ans
}

// Spell Matcher
fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
    let mut sorted_potions = potions.clone();
    sorted_potions.sort_unstable();
    let m = sorted_potions.len() as i64;

    spells.iter().map(|&spell| {
        let min_potion = (success + spell as i64 - 1) / spell as i64;
        let pos = sorted_potions.partition_point(|&p| (p as i64) < min_potion);
        (m - pos as i64) as i32
    }).collect()
}

// Main
fn main() {
    println!("=== Bit Navigator ===");
    println!("S3, k=1 => {}", find_kth_bit(3, 1));
    println!("S4, k=11 => {}", find_kth_bit(4, 11));

    println!("\n=== Flood Shield ===");
    println!("rains=[1,2,3,4]     => {:?}", avoid_flood(vec![1,2,3,4]));
    println!("rains=[1,2,0,0,2,1] => {:?}", avoid_flood(vec![1,2,0,0,2,1]));
    println!("rains=[1,2,0,1,2]   => {:?}", avoid_flood(vec![1,2,0,1,2]));

    println!("\n=== Spell Matcher ===");
    println!("spells=[5,1,3] potions=[1,2,3,4,5] success=7  => {:?}", successful_pairs(vec![5,1,3], vec![1,2,3,4,5], 7));
    println!("spells=[3,1,2] potions=[8,5,8] success=16     => {:?}", successful_pairs(vec![3,1,2], vec![8,5,8], 16));
}
