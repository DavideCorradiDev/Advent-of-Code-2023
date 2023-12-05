#[derive(Debug, Clone)]
struct Input {
    times: Vec<u64>,
    distances: Vec<u64>,
}

impl From<std::fs::File> for Input {
    fn from(file: std::fs::File) -> Self {
        use std::io::{BufRead, BufReader};
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let times = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        let distances = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        Self { times, distances }
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

fn count_solutions(time: u64, distance: u64) -> u64 {
    let tt = time * time;
    let d4 = 4 * distance;

    if tt < d4 {
        0
    } else {
        let diff = ((tt - d4) as f64).sqrt();
        let min = ((time as f64 - diff) / 2.).floor() as u64;
        let max = ((time as f64 + diff) / 2.).ceil() as u64;
        max - min - 1
    }
}

fn part_1(input: &Input) -> u64 {
    let mut solutions = Vec::new();
    for i in 0..input.times.len() {
        solutions.push(count_solutions(input.times[i], input.distances[i]));
    }
    solutions.iter().product()
}

fn concatenate_numbers(v: &Vec<u64>) -> u64 {
    v.iter()
        .map(|x| x.to_string())
        .fold(String::new(), |acc, x| acc + &x)
        .parse::<u64>()
        .unwrap()
}

fn part_2(input: &Input) -> u64 {
    let time = concatenate_numbers(&input.times);
    let distance = concatenate_numbers(&input.distances);
    count_solutions(time, distance)
}

fn main() {
    utils::run::<_, _>(
        &["day06/sample_input.txt", "day06/input.txt"],
        &[part_1, part_2],
    );
}
