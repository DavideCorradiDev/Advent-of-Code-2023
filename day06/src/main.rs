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
    input
        .times
        .iter()
        .zip(input.distances.iter())
        .map(|(x, y)| count_solutions(*x, *y))
        .product()
}

fn concatenate_numbers(v: &Vec<u64>) -> u64 {
    v.iter().rev().fold(0, |acc, x| {
        let mut mult = 1;
        while mult < acc {
            mult *= 10;
        }
        acc + mult * x
    })
}

fn part_2(input: &Input) -> u64 {
    let time = concatenate_numbers(&input.times);
    let distance = concatenate_numbers(&input.distances);
    count_solutions(time, distance)
}

fn main() {
    use utils::PrintMode;
    utils::run::<_, _>(
        &[
            ("day06/sample_input.txt", PrintMode::None),
            ("day06/input.txt", PrintMode::None),
        ],
        &[part_1, part_2],
    );
}
