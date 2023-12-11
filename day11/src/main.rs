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

#[derive(Clone, PartialEq, Eq)]
struct Grid<T> {
    cells: Vec<T>,
    size: Coordinates,
}

impl<T> Grid<T>
where
    T: Clone,
{
    fn coords_to_idx(&self, coords: &Coordinates) -> Option<usize> {
        if coords.row < self.size.row && coords.col < self.size.col {
            Some((self.size.col * coords.row + coords.col) as usize)
        } else {
            None
        }
    }

    pub fn new(cells: Vec<T>, row_count: usize, col_count: usize) -> Self {
        assert!(cells.len() == col_count * row_count);
        Self {
            cells,
            size: Coordinates::new(row_count as i64, col_count as i64),
        }
    }

    pub fn get(&self, coords: &Coordinates) -> Option<T> {
        match self.coords_to_idx(coords) {
            Some(idx) => Some(self.cells[idx].clone()),
            None => None,
        }
    }
}

impl<T> std::fmt::Debug for Grid<T>
where
    T: Clone + std::fmt::Debug,
{
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

#[derive(Debug, Clone)]
struct Input {
    grid: Grid<u64>,
}

impl From<std::fs::File> for Input {
    fn from(file: std::fs::File) -> Self {
        use std::io::{BufRead, BufReader};

        let mut col_count = 0;
        let mut row_count = 0;
        let mut cells = Vec::new();
        let mut galaxy_idx = 1;

        for line in BufReader::new(file).lines().map(|x| x.unwrap()) {
            col_count = line.len();
            row_count += 1;
            cells.extend(line.chars().map(|x| match x {
                '.' => 0,
                '#' => {
                    let code = galaxy_idx;
                    galaxy_idx += 1;
                    code
                }
                _ => panic!("Unrecognized symbol"),
            }));
        }

        Self {
            grid: Grid::new(cells, row_count, col_count),
        }
    }
}

fn compute_distances(grid: &Grid<u64>, skip: i64) -> u64 {
    assert!(skip > 0);

    let empty_rows = {
        let mut ans = Vec::new();
        for row in 0..grid.size.row {
            if (0..grid.size.col).all(|col| grid.get(&Coordinates::new(row, col)).unwrap() == 0) {
                ans.push(row);
            }
        }
        ans
    };

    let empty_cols = {
        let mut ans = Vec::new();
        for col in 0..grid.size.col {
            if (0..grid.size.row).all(|row| grid.get(&Coordinates::new(row, col)).unwrap() == 0) {
                ans.push(col);
            }
        }
        ans
    };

    let galaxies = {
        let mut ans = Vec::new();
        for row in 0..grid.size.row {
            for col in 0..grid.size.col {
                let coords = Coordinates::new(row, col);
                if grid.get(&coords).unwrap() != 0 {
                    ans.push(coords);
                }
            }
        }
        ans
    };

    let distances = {
        let mut ans = std::collections::HashMap::new();
        for i in 0..galaxies.len() {
            for j in (i + 1)..galaxies.len() {
                let min_row = galaxies[i].row.min(galaxies[j].row);
                let max_row = galaxies[i].row.max(galaxies[j].row);
                let min_col = galaxies[i].col.min(galaxies[j].col);
                let max_col = galaxies[i].col.max(galaxies[j].col);
                let empty_rows_count = empty_rows
                    .iter()
                    .filter(|x| **x > min_row && **x < max_row)
                    .count() as i64;
                let empty_cols_count = empty_cols
                    .iter()
                    .filter(|x| **x > min_col && **x < max_col)
                    .count() as i64;
                let distance = max_row - min_row + empty_rows_count * (skip - 1) + max_col
                    - min_col
                    + empty_cols_count * (skip - 1);
                let distance = distance as u64;
                ans.insert(
                    (
                        grid.get(&galaxies[i]).unwrap(),
                        grid.get(&galaxies[j]).unwrap(),
                    ),
                    distance,
                );
            }
        }
        ans
    };

    distances.iter().map(|(_, dist)| dist).sum()
}

fn part_1(input: &Input) -> u64 {
    compute_distances(&input.grid, 2)
}

fn part_2(input: &Input) -> u64 {
    compute_distances(&input.grid, 1000000)
}

fn main() {
    use utils::PrintMode;
    utils::run::<_, _>(
        &[
            ("day11/sample_input.txt", PrintMode::None),
            ("day11/input.txt", PrintMode::None),
        ],
        &[part_1, part_2],
    );
}
