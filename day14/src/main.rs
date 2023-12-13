#[derive(Debug, Clone)]
struct Input {}

impl From<std::fs::File> for Input {
    fn from(file: std::fs::File) -> Self {
        use std::io::{BufRead, BufReader};
        Self {}
    }
}

fn part_1(input: &Input) -> u64 {
    0
}

fn part_2(input: &Input) -> u64 {
    0
}

fn main() {
    use utils::PrintMode;
    utils::run::<_, _>(
        &[
            ("day14/sample_input.txt", PrintMode::Debug),
            ("day14/input.txt", PrintMode::None),
        ],
        &[part_1, part_2],
    );
}