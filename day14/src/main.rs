#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coordinates {
    row: i64,
    col: i64,
}

impl Coordinates {
    pub fn new(row: i64, col: i64) -> Self {
        Self { row, col }
    }
}

impl std::ops::Add for Coordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl std::ops::Sub for Coordinates {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row - rhs.row,
            col: self.col - rhs.col,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tilt {
    West,
    East,
    North,
    South,
}

#[derive(Clone, PartialEq, Eq)]
struct Grid {
    cells: Vec<char>,
    size: Coordinates,
}

impl Grid {
    fn coords_to_idx(&self, coords: &Coordinates) -> Option<usize> {
        if coords.row < self.size.row && coords.col < self.size.col {
            Some((self.size.col * coords.row + coords.col) as usize)
        } else {
            None
        }
    }

    pub fn new(cells: Vec<char>, row_count: usize, col_count: usize) -> Self {
        assert!(cells.len() == col_count * row_count);
        Self {
            cells,
            size: Coordinates::new(row_count as i64, col_count as i64),
        }
    }

    pub fn get(&self, coords: &Coordinates) -> Option<char> {
        match self.coords_to_idx(coords) {
            Some(idx) => Some(self.cells[idx].clone()),
            None => None,
        }
    }

    pub fn add_walls(&mut self) {
        let mut cells = Vec::new();
        cells.append(&mut vec!['#'; self.size.col as usize + 2]);
        for row in 0..self.size.row {
            cells.push('#');
            cells.extend_from_slice(
                &self.cells[(row as usize * self.size.col as usize)
                    ..((row as usize + 1) * self.size.col as usize)],
            );
            cells.push('#');
        }
        cells.append(&mut vec!['#'; self.size.col as usize + 2]);
        self.cells = cells;
        self.size.row += 2;
        self.size.col += 2;
    }

    pub fn transpose(&self) -> Self {
        let mut cells = Vec::new();
        for row in 0..self.size.row {
            for col in 0..self.size.col {
                cells.push(self.get(&Coordinates::new(col, row)).unwrap());
            }
        }
        Self {
            cells,
            size: Coordinates::new(self.size.col, self.size.row),
        }
    }

    pub fn tilt(&mut self, tilt: Tilt) {
        let transpose = tilt == Tilt::South || tilt == Tilt::North;
        let get_val = match tilt {
            Tilt::West | Tilt::North => |x: &char| if *x == 'O' { 0 } else { 1 },
            Tilt::East | Tilt::South => |x: &char| if *x == 'O' { 1 } else { 0 },
        };

        if transpose {
            *self = self.transpose();
        }

        let mut walls: Vec<usize> = self
            .cells
            .iter()
            .enumerate()
            .filter(|(_, x)| **x == '#')
            .map(|(idx, _)| idx)
            .collect();
        walls.push(self.cells.len());

        let mut ranges = Vec::new();
        let mut start = 0;
        for wall_idx in walls.into_iter() {
            if wall_idx > start {
                ranges.push((start, wall_idx));
            }
            start = wall_idx + 1;
        }

        for (start, end) in ranges.into_iter() {
            self.cells[start..end].sort_by(|x, y| get_val(x).cmp(&get_val(y)));
        }

        if transpose {
            *self = self.transpose();
        }
    }

    pub fn cycle(&mut self) {
        self.tilt(Tilt::North);
        self.tilt(Tilt::West);
        self.tilt(Tilt::South);
        self.tilt(Tilt::East);
    }

    pub fn multicycle(&mut self, count: usize) {
        use std::collections::HashMap;
        let mut cache = HashMap::new();
        let mut i = 0;
        while i < count {
            if let Some(idx) = cache.get(&self.cells) {
                println!("Cache hit at {i} from {idx}");
                let step = i - idx;
                i += (count - i) / step * step;
                while i < count {
                    self.cycle();
                    i += 1;
                }
            } else {
                cache.insert(self.cells.clone(), i);
                self.cycle();
                i += 1;
            }
        }
    }

    pub fn calculate_load(&self) -> u64 {
        let mut load = 0;
        for i in 0..self.size.row {
            for j in 0..self.size.col {
                if self.get(&Coordinates::new(i, j)).unwrap() == 'O' {
                    load += self.size.row - 1 - i;
                }
            }
        }
        load as u64
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for row in 0..self.size.row {
            for col in 0..self.size.col {
                write!(f, "{:?}", self.get(&Coordinates::new(row, col)).unwrap())?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for row in 0..self.size.row {
            for col in 0..self.size.col {
                write!(f, "{}", self.get(&Coordinates::new(row, col)).unwrap())?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl From<std::fs::File> for Grid {
    fn from(file: std::fs::File) -> Self {
        use std::io::{BufRead, BufReader};
        let mut grid = Self::new(Vec::new(), 0, 0);
        for line in BufReader::new(file).lines().map(Result::unwrap) {
            grid.size.row += 1;
            grid.size.col = line.len() as i64;
            grid.cells.append(&mut line.chars().collect());
        }
        grid.add_walls();
        grid
    }
}

fn part_1(input: &Grid) -> u64 {
    let mut grid = input.clone();
    grid.tilt(Tilt::North);
    grid.calculate_load()
}

fn part_2(input: &Grid) -> u64 {
    let mut grid = input.clone();
    grid.multicycle(1000000000);
    grid.calculate_load()
}

fn main() {
    use utils::PrintMode;
    utils::run::<_, _>(
        &[
            ("day14/sample_input.txt", PrintMode::None),
            ("day14/input.txt", PrintMode::None),
        ],
        &[part_1, part_2],
    );
}
