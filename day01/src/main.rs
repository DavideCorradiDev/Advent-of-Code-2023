#[derive(Debug, Clone)]
struct Input {
    data: Vec<String>,
}

impl From<std::fs::File> for Input {
    fn from(file: std::fs::File) -> Self {
        use std::io::{BufRead, BufReader};
        let data = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
        Self { data }
    }
}

fn part_1(input: &Input) -> Option<u32> {
    let find_digit = |x: &char| {
        return *x >= '0' && *x <= '9';
    };

    let mut ans = 0;

    for line in input.data.iter() {
        let first_digit = match line.chars().find(find_digit) {
            Some(digit) => digit.to_digit(10).unwrap(),
            None => return None,
        };
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

    Some(ans)
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

fn part_2(input: &Input) -> Option<u32> {
    let mut ans = 0;

    for line in input.data.iter() {
        let digits = find_digits(line);
        let first_digit = digits.0;
        let last_digit = digits.1;
        let calibration_value = 10 * first_digit + last_digit;
        ans += calibration_value;
    }

    Some(ans)
}

fn main() {
    use utils::PrintMode;
    utils::run::<_, _>(
        &[
            ("day01/sample_input_1.txt", PrintMode::None),
            ("day01/sample_input_2.txt", PrintMode::None),
            ("day01/input.txt", PrintMode::None),
        ],
        &[part_1, part_2],
    );
}
