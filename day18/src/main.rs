#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec2i {
    x: i64,
    y: i64,
}

impl Vec2i {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl std::ops::AddAssign for Vec2i {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Add for Vec2i {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl std::ops::SubAssign for Vec2i {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Sub for Vec2i {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl std::ops::MulAssign<i64> for Vec2i {
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl std::ops::Mul<i64> for Vec2i {
    type Output = Self;
    fn mul(mut self, rhs: i64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl std::ops::Mul<Vec2i> for i64 {
    type Output = Vec2i;
    fn mul(self, rhs: Vec2i) -> Self::Output {
        rhs * self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Polygon(Vec<Vec2i>);

impl Polygon {
    fn get(&self, idx: usize) -> &Vec2i {
        &self.0[idx % self.0.len()]
    }

    fn area(&self) -> i64 {
        let mut double_area = 0;
        for i in 0..self.0.len() {
            double_area += self.get(i).x * self.get(i + 1).y - self.get(i + 1).x * self.get(i).y;
        }
        double_area.abs() / 2
    }
}

impl From<Vec<Instruction>> for Polygon {
    fn from(instructions: Vec<Instruction>) -> Self {
        let mut poly = Polygon(Vec::with_capacity(instructions.len() + 1));
        poly.0.push(Vec2i::new(0, 0));
        instructions
            .into_iter()
            .for_each(|Instruction { dir, len }| {
                poly.0
                    .push(*poly.0.last().unwrap() + len * Vec2i::from(dir));
            });
        poly
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

impl From<Direction> for Vec2i {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Down => Vec2i::new(0, -1),
            Direction::Up => Vec2i::new(0, 1),
            Direction::Left => Vec2i::new(-1, 0),
            Direction::Right => Vec2i::new(1, 0),
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            'U' => Self::Up,
            'D' => Self::Down,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    dir: Direction,
    len: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Entry {
    dir: char,
    len: String,
    col: String,
}

impl Entry {
    fn instruction(&self) -> Instruction {
        Instruction {
            dir: Direction::from(self.dir),
            len: i64::from_str_radix(&self.len, 10).unwrap(),
        }
    }

    fn color_instruction(&self) -> Instruction {
        let dir = match self.col.chars().last().unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => panic!("Invalid direction"),
        };
        let len = i64::from_str_radix(&self.col[0..5], 16).unwrap();
        Instruction { dir, len }
    }
}

#[derive(Debug, Clone)]
struct Input {
    entries: Vec<Entry>,
}

impl From<std::fs::File> for Input {
    fn from(file: std::fs::File) -> Self {
        use std::io::{BufRead, BufReader};
        let entries = BufReader::new(file)
            .lines()
            .map(Result::unwrap)
            .map(|line| {
                let (dir, len, col) = {
                    let (dir, len) = line.split_once(" ").unwrap();
                    let (len, col) = len.split_once(" ").unwrap();
                    let dir = dir.chars().next().unwrap();
                    let len = String::from(len);
                    let col = String::from(&col[2..col.len() - 1]);
                    (dir, len, col)
                };
                Entry { dir, len, col }
            })
            .collect();
        Self { entries }
    }
}

fn part_1(input: &Input) -> i64 {
    let instructions: Vec<Instruction> = input.entries.iter().map(|x| x.instruction()).collect();
    let perimeter: i64 = instructions.iter().map(|Instruction{dir: _, len}| len).sum();
    let area = Polygon::from(instructions).area();
    area + perimeter / 2 + 1
}

fn part_2(input: &Input) -> i64 {
    let instructions: Vec<Instruction> = input
        .entries
        .iter()
        .map(|x| x.color_instruction())
        .collect();
    let perimeter: i64 = instructions.iter().map(|Instruction{dir: _, len}| len).sum();
    let area = Polygon::from(instructions).area();
    area + perimeter / 2 + 1
}

fn main() {
    use utils::PrintMode;
    utils::run::<_, _>(
        &[
            ("day18/sample_input.txt", PrintMode::None),
            ("day18/custom_input_1.txt", PrintMode::None),
            ("day18/input.txt", PrintMode::None),
        ],
        &[part_1, part_2],
    );
}
