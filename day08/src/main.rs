use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left = 0,
    Right = 1,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unrecognized direction"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Input {
    directions: Vec<Direction>,
    map: HashMap<String, [String; 2]>,
}

impl From<std::fs::File> for Input {
    fn from(file: std::fs::File) -> Self {
        use std::io::{BufRead, BufReader};

        let mut lines = BufReader::new(file).lines();

        let directions = lines
            .next()
            .unwrap()
            .unwrap()
            .chars()
            .map(|x| Direction::from(x))
            .collect();

        let mut map = HashMap::new();
        for line in lines {
            let line = line.unwrap();
            if line.is_empty() {
                continue;
            }
            let (start_str, left_and_right_str) = line.split_once(" = ").unwrap();
            let (left_str, right_str) = left_and_right_str[1..left_and_right_str.len() - 1]
                .split_once(", ")
                .unwrap();
            map.insert(
                String::from(start_str),
                [String::from(left_str), String::from(right_str)],
            );
        }

        Self { directions, map }
    }
}

fn step_forward<'a>(it: u64, current_location: &str, input: &'a Input) -> Option<&'a str> {
    let direction = input.directions[it as usize % input.directions.len()];
    match input.map.get(current_location) {
        Some(entry) => Some(&entry[direction as usize]),
        None => None,
    }
}

fn count_steps_to_solution(current_location: &str, input: &Input) -> Option<u64> {
    let mut current_location = current_location;
    let mut i = 0;
    while !current_location.ends_with("Z") {
        match step_forward(i, &current_location, input) {
            Some(location) => current_location = &location,
            None => return None,
        }
        i += 1;
    }
    Some(i)
}

fn part_1(input: &Input) -> Option<u64> {
    count_steps_to_solution("AAA", input)
}

fn gcd(mut x: u64, mut y: u64) -> u64 {
    if y > x {
        std::mem::swap(&mut x, &mut y);
    }
    while y > 0 {
        let modulo = x % y;
        x = y;
        y = modulo;
    }
    x
}

fn lcm(x: u64, y: u64) -> u64 {
    x * y / gcd(x, y)
}

fn multi_lcm(v: &Vec<u64>) -> u64 {
    let mut ans = v[0];
    for i in 1..v.len() {
        ans = lcm(ans, v[i]);
    }
    ans
}

fn part_2(input: &Input) -> Option<u64> {
    let steps = input
        .map
        .keys()
        .filter(|x| x.ends_with("A"))
        .map(|x| count_steps_to_solution(x, input).unwrap())
        .collect();
    Some(multi_lcm(&steps))
}

fn main() {
    use utils::PrintMode;
    utils::run::<_, _>(
        &[
            ("day08/sample_input_1.txt", PrintMode::None),
            ("day08/sample_input_2.txt", PrintMode::None),
            ("day08/sample_input_3.txt", PrintMode::None),
            ("day08/input.txt", PrintMode::None),
        ],
        &[part_1, part_2],
    );
}
