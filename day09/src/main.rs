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

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let max_len = 8;
        if self.sequences.len() > max_len {
            let mut c = Input {
                sequences: Vec::new(),
            };
            for i in 0..max_len {
                c.sequences.push(self.sequences[i].clone());
            }
            write!(f, "{:?} (Input was clamped because too large)", c)
        } else {
            write!(f, "{:?}", self)
        }
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
    utils::run::<_, _>(
        &["day09/sample_input.txt", "day09/input.txt"],
        &[part_1, part_2],
    );
}
