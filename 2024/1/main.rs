use std::collections::HashMap;
use std::fs;

fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("inputs.txt")?;
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in content.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if nums.len() == 2 {
            left_list.push(nums[0]);
            right_list.push(nums[1]);
        }
    }

    if left_list.len() != right_list.len() {
        eprintln!("Les listes n'ont pas la même taille !");
        return Ok(());
    }

    // PART 1: Total distance
    let mut left_sorted = left_list.clone();
    let mut right_sorted = right_list.clone();
    left_sorted.sort();
    right_sorted.sort();

    let total_distance: i64 = left_sorted
        .iter()
        .zip(right_sorted.iter())
        .map(|(a, b)| (*a - *b).abs() as i64)
        .sum();

    println!("Distance totale (Partie 1) : {}", total_distance);

    // PART 2: Similarity score
    let mut right_count = HashMap::new();
    for &val in &right_list {
        *right_count.entry(val).or_insert(0) += 1;
    }

    let similarity_score: i64 = left_list
        .iter()
        .map(|&val| val as i64 * *right_count.get(&val).unwrap_or(&0) as i64)
        .sum();

    println!("Similarity score (Partie 2) : {}", similarity_score);

    Ok(())
}