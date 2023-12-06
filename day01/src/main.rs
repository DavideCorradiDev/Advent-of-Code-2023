use std::fs::File;
use std::io::{BufRead, BufReader};

fn first_part() {
    let file = File::open("day01/data/input.txt").expect("Unable to open the input file");
    let reader = BufReader::new(file);
    let find_digit = |x: &char| {
        return *x >= '0' && *x <= '9';
    };
    let mut ans = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let first_digit = line.chars().find(find_digit).unwrap().to_digit(10).unwrap();
        let last_digit = line
            .chars()
            .rev()
            .find(find_digit)
            .unwrap()
            .to_digit(10)
            .unwrap();
        let calibration_value = 10 * first_digit + last_digit;
        ans += calibration_value;
    }

    println!("The total calibration value is '{ans}'.")
}

fn starts_with_digit(s: &str) -> Option<u32> {
    if s.starts_with("one") || s.starts_with("1") {
        Some(1)
    } else if s.starts_with("two") || s.starts_with("2") {
        Some(2)
    } else if s.starts_with("three") || s.starts_with("3") {
        Some(3)
    } else if s.starts_with("four") || s.starts_with("4") {
        Some(4)
    } else if s.starts_with("five") || s.starts_with("5") {
        Some(5)
    } else if s.starts_with("six") || s.starts_with("6") {
        Some(6)
    } else if s.starts_with("seven") || s.starts_with("7") {
        Some(7)
    } else if s.starts_with("eight") || s.starts_with("8") {
        Some(8)
    } else if s.starts_with("nine") || s.starts_with("9") {
        Some(9)
    } else {
        None
    }
}

fn find_digits(s: &String) -> (u32, u32) {
    let mut ans: Option<(u32, u32)> = None;

    for i in 0..s.len() {
        if let Some(digit) = starts_with_digit(&s[i..]) {
            if let Some(ans) = &mut ans {
                ans.1 = digit;
            } else {
                ans = Some((digit, digit))
            }
        }
    }

    ans.unwrap()
}

fn second_part() {
    let file = File::open("day01/data/input.txt").expect("Unable to open the input file");
    let reader = BufReader::new(file);
    let mut ans = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let digits = find_digits(&line);
        let first_digit = digits.0;
        let last_digit = digits.1;
        let calibration_value = 10 * first_digit + last_digit;
        ans += calibration_value;
    }

    println!("The total calibration value is '{ans}'.")
}

fn main() {
    first_part();
    second_part();
}
