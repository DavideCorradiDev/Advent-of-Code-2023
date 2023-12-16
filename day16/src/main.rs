use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    cells: Vec<char>,
    size: Coordinates,
}

impl Grid {
    fn coords_to_idx(&self, coords: &Coordinates) -> Option<usize> {
        if coords.row >= 0
            && coords.row < self.size.row
            && coords.col >= 0
            && coords.col < self.size.col
        {
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

    pub fn set(&mut self, coords: &Coordinates, val: char) {
        match self.coords_to_idx(coords) {
            Some(idx) => self.cells[idx] = val,
            None => panic!("Invalid idx"),
        }
    }

    pub fn find_energized_tiles(&self, pos: Coordinates, dir: Coordinates) -> Grid {
        let mut cache = HashSet::new();
        let mut energized_tiles = Grid::new(
            vec!['.'; self.cells.len()],
            self.size.row as usize,
            self.size.col as usize,
        );
        self.project_beam(&mut cache, &mut energized_tiles, pos, dir);
        energized_tiles
    }

    pub fn project_beam(
        &self,
        cache: &mut HashSet<(Coordinates, Coordinates)>,
        energised_tiles: &mut Grid,
        mut pos: Coordinates,
        mut dir: Coordinates,
    ) {
        loop {
            if let Some(tile) = self.get(&pos) {
                cache.insert((pos, dir));
                energised_tiles.set(&pos, '#');
                match tile {
                    '.' => {
                        pos = pos + dir;
                    }
                    '/' => {
                        std::mem::swap(&mut dir.row, &mut dir.col);
                        dir.row *= -1;
                        dir.col *= -1;
                        pos = pos + dir;
                    }
                    '\\' => {
                        std::mem::swap(&mut dir.row, &mut dir.col);
                        pos = pos + dir;
                    }
                    '|' => {
                        if dir.col != 0 {
                            for dir in [Coordinates::new(-1, 0), Coordinates::new(1, 0)] {
                                let pos = pos + dir;
                                if let None = cache.get(&(pos, dir)) {
                                    self.project_beam(cache, energised_tiles, pos, dir)
                                }
                            }
                            return;
                        } else {
                            pos = pos + dir;
                        }
                    }
                    '-' => {
                        if dir.row != 0 {
                            for dir in [Coordinates::new(0, -1), Coordinates::new(0, 1)] {
                                let pos = pos + dir;
                                if let None = cache.get(&(pos, dir)) {
                                    self.project_beam(cache, energised_tiles, pos, dir)
                                }
                            }
                            return;
                        } else {
                            pos = pos + dir;
                        }
                    }
                    _ => panic!("Invalid character"),
                }
            } else {
                return;
            }
        }
    }

    fn count_energised_tiles(&self) -> usize {
        self.cells.iter().filter(|x| **x == '#').count()
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
        grid
    }
}

fn part_1(grid: &Grid) -> usize {
    grid.find_energized_tiles(Coordinates::new(0, 0), Coordinates::new(0, 1))
        .count_energised_tiles()
}

fn part_2(grid: &Grid) -> usize {
    let mut starting_configs = Vec::new();
    for row in 0..grid.size.row {
        starting_configs.push((Coordinates::new(row, 0), Coordinates::new(0, 1)));
        starting_configs.push((
            Coordinates::new(row, grid.size.col - 1),
            Coordinates::new(0, -1),
        ));
    }
    for col in 0..grid.size.col {
        starting_configs.push((Coordinates::new(0, col), Coordinates::new(1, 0)));
        starting_configs.push((
            Coordinates::new(grid.size.row - 1, col),
            Coordinates::new(-1, 0),
        ));
    }
    starting_configs
        .into_iter()
        .map(|(pos, dir)| grid.find_energized_tiles(pos, dir).count_energised_tiles())
        .max()
        .unwrap()
}

fn main() {
    use utils::PrintMode;
    utils::run::<_, _>(
        &[
            ("day16/sample_input.txt", PrintMode::None),
            ("day16/input.txt", PrintMode::None),
        ],
        &[part_1, part_2],
    );
}
