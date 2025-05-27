use std::fs;

fn compute_score_str(s: &str) -> u64 {
    let mut out = 0;
    for c in s.chars() {
        out = ((out + c as u64 ) * 17)%256;
    }
    out
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut total = 0;
    for i in input.trim().split(',') {
        total += compute_score_str(i);
    }
    println!("{}", total);
}
