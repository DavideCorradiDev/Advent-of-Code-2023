#[derive(Debug, Clone)]
struct Input {}

impl From<std::fs::File> for Input {
    fn from(file: std::fs::File) -> Self {
        Self {}
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

fn part_1(input: &Input) -> u64 {
    0
}

fn part_2(input: &Input) -> u64 {
    0
}

fn main() {
    utils::run::<_, _>(
        &["day07/sample_input.txt", "day07/input.txt"],
        &[part_1, part_2],
    );
}
