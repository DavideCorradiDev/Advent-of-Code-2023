#[derive(Debug, Clone)]
struct Input {
    sequences: Vec<Vec<i64>>,
}

impl From<std::fs::File> for Input {
    fn from(file: std::fs::File) -> Self {
        use std::io::{BufRead, BufReader};
        let mut sequences = Vec::new();
        for line in BufReader::new(file).lines() {
            sequences.push(
                line.unwrap()
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect(),
            );
        }
        Self { sequences }
    }
}

fn part_1(input: &Input) -> i64 {
    let mut sequence_results = Vec::with_capacity(input.sequences.len());

    for mut sequence in input.sequences.iter().cloned() {
        let mut end_idx = sequence.len();

        while end_idx > 0 && !sequence[0..end_idx].iter().all(|x| *x == 0) {
            for next in 1..end_idx {
                let prev = next - 1;
                sequence[prev] = sequence[next] - sequence[prev];
            }
            end_idx -= 1;
        }

        sequence_results.push(sequence[(end_idx - 1)..sequence.len()].iter().sum());
    }

    sequence_results.iter().sum()
}

fn part_2(input: &Input) -> i64 {
    let mut sequence_results = Vec::with_capacity(input.sequences.len());

    for mut sequence in input.sequences.iter().cloned() {
        let mut end_idx = sequence.len();
        sequence.reverse();

        while end_idx > 0 && !sequence[0..end_idx].iter().all(|x| *x == 0) {
            for next in 1..end_idx {
                let prev = next - 1;
                sequence[prev] = sequence[prev] - sequence[next];
            }
            end_idx -= 1;
        }

        sequence_results.push(
            sequence[(end_idx - 1)..sequence.len()]
                .iter()
                .fold(0, |acc, x| x - acc),
        );
    }

    sequence_results.iter().sum()
}

fn main() {
    use utils::PrintMode;
    utils::run::<_, _>(
        &[
            ("day09/sample_input.txt", PrintMode::None),
            ("day09/input.txt", PrintMode::None),
        ],
        &[part_1, part_2],
    );
}
