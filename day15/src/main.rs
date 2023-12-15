#[derive(Debug, Clone)]
struct Input {
    steps: Vec<String>,
}

impl From<std::fs::File> for Input {
    fn from(mut file: std::fs::File) -> Self {
        use std::io::Read;

        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)
            .expect("Couldn't read input");
        let steps = file_contents.split(",").map(String::from).collect();
        Self { steps }
    }
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, x| (acc + x as usize) * 17 % 256)
}

fn part_1(input: &Input) -> usize {
    input.steps.iter().map(|x| hash(&x)).sum()
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: u8,
}

#[derive(Debug, Clone)]
enum Step {
    Set(Lens),
    Remove(String),
}

impl From<&str> for Step {
    fn from(s: &str) -> Self {
        if s.ends_with("-") {
            Self::Remove(String::from(&s[0..(s.len() - 1)]))
        } else {
            let (label, focal_length) = s.split_once("=").unwrap();
            Self::Set(Lens {
                label: String::from(label),
                focal_length: focal_length.parse::<u8>().unwrap(),
            })
        }
    }
}

fn part_2(input: &Input) -> usize {
    let mut boxes = vec![Vec::<Lens>::new(); 256];
    for step in input.steps.iter().map(|x| Step::from(&x[..])) {
        match step {
            Step::Remove(label) => {
                let box_idx = hash(&label);
                if let Some(lens_idx) = boxes[box_idx].iter().position(|x| x.label == label) {
                    boxes[box_idx].remove(lens_idx);
                }
            }
            Step::Set(lens) => {
                let box_idx = hash(&lens.label);
                if let Some(lens_idx) = boxes[box_idx].iter().position(|x| x.label == lens.label) {
                    boxes[box_idx][lens_idx] = lens;
                } else {
                    boxes[box_idx].push(lens);
                }
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(box_idx, boxx)| {
            (1 + box_idx)
                * boxx
                    .iter()
                    .enumerate()
                    .map(|(lens_idx, lens)| (1 + lens_idx) * lens.focal_length as usize)
                    .sum::<usize>()
        })
        .sum()
}

fn main() {
    use utils::PrintMode;
    utils::run::<_, _>(
        &[
            ("day15/sample_input.txt", PrintMode::None),
            ("day15/input.txt", PrintMode::None),
        ],
        &[part_1, part_2],
    );
}
