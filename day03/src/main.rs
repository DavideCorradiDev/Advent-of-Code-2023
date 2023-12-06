use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input(file: File) -> Vec<Vec<char>> {
    let mut ans = Vec::new();
    for line in BufReader::new(file).lines() {
        let line = line.expect("Error reading line from file");
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        ans.push(row);
    }
    ans
}

fn is_symbol(c: char) -> bool {
    c != '.' && !char::is_alphanumeric(c)
}

fn is_next_to_symbol(data: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i > 0 {
        let row = &data[i - 1];
        if j > 0 && is_symbol(row[j - 1]) {
            return true;
        }
        if is_symbol(row[j]) {
            return true;
        }
        if j + 1 < row.len() && is_symbol(row[j + 1]) {
            return true;
        }
    }
    if i + 1 < data.len() {
        let row = &data[i + 1];
        if j > 0 && is_symbol(row[j - 1]) {
            return true;
        }
        if is_symbol(row[j]) {
            return true;
        }
        if j + 1 < row.len() && is_symbol(row[j + 1]) {
            return true;
        }
    }
    let row = &data[i];
    if j > 0 && is_symbol(row[j - 1]) {
        return true;
    }
    if j + 1 < row.len() && is_symbol(row[j + 1]) {
        return true;
    }
    false
}

fn first_part() {
    let input = read_input(File::open("day03/data/input.txt").expect("Couldn't open input file"));

    let mut ans = 0;

    for i in 0..input.len() {
        let row = &input[i];
        let mut j = 0;
        while j < row.len() {
            if char::is_numeric(row[j]) && is_next_to_symbol(&input, i, j) {
                while j > 0 && char::is_numeric(row[j - 1]) {
                    j -= 1;
                }
                let mut number = 0;
                while j < row.len() && char::is_numeric(row[j]) {
                    number = 10 * number + row[j].to_digit(10).unwrap();
                    j += 1;
                }
                ans += number;
            } else {
                j += 1;
            }
        }
    }

    println!("The answer is '{ans}'");
}

fn build_number(data: &Vec<char>, j: usize) -> Option<u32> {
    if !char::is_numeric(data[j]) {
        None
    } else {
        let mut start = j;
        while start > 0 && char::is_numeric(data[start - 1]) {
            start -= 1;
        }
        let mut end = j + 1;
        while end < data.len() && char::is_numeric(data[end]) {
            end += 1;
        }
        let mut number = 0;
        while start != end {
            number = 10 * number + data[start].to_digit(10).unwrap();
            start += 1;
        }
        Some(number)
    }
}

fn second_part() {
    let input = read_input(File::open("day03/data/input.txt").expect("Couldn't open input file"));

    let mut ans = 0;

    for i in 0..input.len() {
        let row = &input[i];
        for j in 0..row.len() {
            if row[j] == '*' {
                let mut adjacent_numbers = Vec::new();
                if j > 0 {
                    if let Some(number) = build_number(row, j - 1) {
                        adjacent_numbers.push(number);
                    }
                }
                if j + 1 < row.len() {
                    if let Some(number) = build_number(row, j + 1) {
                        adjacent_numbers.push(number);
                    }
                }
                if i > 0 {
                    let row = &input[i - 1];
                    if let Some(number) = build_number(row, j) {
                        adjacent_numbers.push(number);
                    } else {
                        if j > 0 {
                            if let Some(number) = build_number(row, j - 1) {
                                adjacent_numbers.push(number);
                            }
                        }
                        if j + 1 < row.len() {
                            if let Some(number) = build_number(row, j + 1) {
                                adjacent_numbers.push(number);
                            }
                        }
                    }
                }
                if i + 1 < input.len() {
                    let row = &input[i + 1];
                    if let Some(number) = build_number(row, j) {
                        adjacent_numbers.push(number);
                    } else {
                        if j > 0 {
                            if let Some(number) = build_number(row, j - 1) {
                                adjacent_numbers.push(number);
                            }
                        }
                        if j + 1 < row.len() {
                            if let Some(number) = build_number(row, j + 1) {
                                adjacent_numbers.push(number);
                            }
                        }
                    }
                }
                if adjacent_numbers.len() == 2 {
                    ans += adjacent_numbers[0] * adjacent_numbers[1];
                }
            }
        }
    }

    println!("The answer is '{ans}'");
}

fn main() {
    first_part();
    second_part();
}
