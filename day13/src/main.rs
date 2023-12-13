#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellKind {
    Ash,
    Rocks,
}

impl CellKind {
    fn switch(&mut self) {
        *self = match self {
            Self::Ash => Self::Rocks,
            Self::Rocks => Self::Ash,
        };
    }
}

impl From<char> for CellKind {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Ash,
            '#' => Self::Rocks,
            _ => panic!("Invalid character"),
        }
    }
}

#[derive(Debug, Clone)]
struct Entry {
    cells: Vec<Vec<CellKind>>,
}

impl Entry {
    fn new() -> Self {
        Entry { cells: Vec::new() }
    }
}

#[derive(Debug, Clone)]
struct Input {
    entries: Vec<Entry>,
}

impl From<std::fs::File> for Input {
    fn from(file: std::fs::File) -> Self {
        use std::io::{BufRead, BufReader};

        let mut entries: Vec<Entry> = Vec::new();
        entries.push(Entry::new());
        for line in BufReader::new(file).lines().map(Result::unwrap) {
            if line.is_empty() {
                entries.push(Entry::new());
            } else {
                entries
                    .last_mut()
                    .unwrap()
                    .cells
                    .push(line.chars().map(|x| CellKind::from(x)).collect());
            }
        }

        Self { entries }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Axis {
    Col(usize),
    Row(usize),
}

impl Axis {
    fn points(&self) -> usize {
        match self {
            Self::Col(x) => *x,
            Self::Row(x) => 100 * x,
        }
    }
}

fn transpose(cells: &Vec<Vec<CellKind>>) -> Vec<Vec<CellKind>> {
    let in_rows = cells.len();
    let in_cols = cells[0].len();
    let mut ans = Vec::with_capacity(in_cols);
    for i in 0..in_cols {
        ans.push(Vec::with_capacity(in_rows));
        for j in 0..in_rows {
            ans.last_mut().unwrap().push(cells[j][i]);
        }
    }
    ans
}

fn is_symmetric(v: &Vec<CellKind>, axis_pos: usize) -> bool {
    (0..axis_pos).all(|i| {
        let mirror_idx = 2 * axis_pos - 1 - i;
        mirror_idx >= v.len() || v[i] == v[mirror_idx]
    })
}

fn find_vertical_symmetries(cells: &Vec<Vec<CellKind>>) -> Vec<usize> {
    let mut ans = Vec::new();
    for i in 1..cells[0].len() {
        if cells.iter().all(|x| is_symmetric(x, i)) {
            ans.push(i);
        }
    }
    ans
}

fn find_horizontal_symmetries(cells: &Vec<Vec<CellKind>>) -> Vec<usize> {
    find_vertical_symmetries(&transpose(&cells))
}

fn find_symmetries(cells: &Vec<Vec<CellKind>>) -> Vec<Axis> {
    let vs = find_vertical_symmetries(&cells);
    let hs = find_horizontal_symmetries(&cells);
    let mut ans: Vec<Axis> = vs.iter().map(|x| Axis::Col(*x)).collect();
    ans.append(&mut hs.iter().map(|x| Axis::Row(*x)).collect());
    ans
}

fn find_symmetry(cells: &Vec<Vec<CellKind>>) -> Axis {
    let symmetries = find_symmetries(cells);
    assert!(symmetries.len() == 1);
    symmetries[0]
}

fn part_1(input: &Input) -> usize {
    input
        .entries
        .iter()
        .map(|x| find_symmetry(&x.cells).points())
        .sum()
}

fn find_symmetry_with_smudge(cells: &Vec<Vec<CellKind>>) -> Axis {
    let orig_symmetry = find_symmetry(cells);

    let mut cells = cells.clone();
    for i in 0..cells.len() {
        for j in 0..cells[0].len() {
            cells[i][j].switch();
            let new_symmetries = find_symmetries(&cells);
            if let Some(new_symmetry) = new_symmetries.into_iter().find(|x| *x != orig_symmetry) {
                return new_symmetry;
            }
            cells[i][j].switch();
        }
    }

    panic!("No new symmetry found!");
}

fn part_2(input: &Input) -> usize {
    input
        .entries
        .iter()
        .map(|x| find_symmetry_with_smudge(&x.cells).points())
        .sum()
}

fn main() {
    use utils::PrintMode;
    utils::run::<_, _>(
        &[
            ("day13/sample_input.txt", PrintMode::None),
            ("day13/input.txt", PrintMode::None),
        ],
        &[part_1, part_2],
    );
}
