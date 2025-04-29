use std::fs;

fn is_safe(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return false;
    }
    let mut increasing = true;
    let mut decreasing = true;
    for w in levels.windows(2) {
        let diff = w[1] - w[0];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
        if diff > 0 { decreasing = false; }
        if diff < 0 { increasing = false; }
    }
    increasing || decreasing
}

fn is_safe_with_one_removal(levels: &[i32]) -> bool {
    for i in 0..levels.len() {
        let mut temp = Vec::with_capacity(levels.len() - 1);
        for (j, &val) in levels.iter().enumerate() {
            if i != j {
                temp.push(val);
            }
        }
        if is_safe(&temp) {
            return true;
        }
    }
    false
}

fn main() {
    let content = fs::read_to_string("inputs.txt").expect("Failed to read file");
    let mut safe_count_part1 = 0;
    let mut safe_count_part2 = 0;

    for line in content.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if is_safe(&levels) {
            safe_count_part1 += 1;
            safe_count_part2 += 1;
        } else if is_safe_with_one_removal(&levels) {
            safe_count_part2 += 1;
        }
    }

    println!("Part 1: {}", safe_count_part1);
    println!("Part 2: {}", safe_count_part2);
}