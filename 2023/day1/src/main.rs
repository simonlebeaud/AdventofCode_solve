use std::{fs, ops::Mul};

fn parse_file(path: &str) -> String {
    let input = fs::read_to_string(path).unwrap();
    input
}



fn calib_value_step1(line:  &str) -> u32 {
    let mut out = 0;
    let digits: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();
    match digits.len() {
        2.. => {
            out += digits[0].to_digit(10).unwrap().mul(10);
            out += digits[digits.len() -1].to_digit(10).unwrap();
        },
        1 => {
            out += digits[0].to_digit(10).unwrap().mul(10);
            out += digits[0].to_digit(10).unwrap();
        },
        0 => println!("No digit in the line"),
    }
    out
}

fn calib_value_step2(line:  &str) -> u32 {
    let new_line = line.to_string()
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine");
    calib_value_step1(&new_line)
}

fn main() {
    // let input = "1abc2
    //         pqr3stu8vwx
    //         a1b2c3d4e5f
    //         treb7uchet";
    // let input = 
    //     "two1nine
    //     eightwothree
    //     abcone2threexyz
    //     xtwone3four
    //     4nineeightseven2
    //     zoneight234
    //     7pqrstsixteen";
    let input = parse_file("input.txt");
    let mut total1 = 0;
    let mut total2 = 0;
    for line in input.lines() {
        total1 += calib_value_step1(line);
        total2 += calib_value_step2(line);
    }

    println!("total step 1 = {}", total1);
    println!("total step 2 = {}", total2);
}
