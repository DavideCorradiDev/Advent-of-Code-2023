use std::cmp::Ordering;
use std::collections::BinaryHeap;
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

impl std::ops::Mul<i64> for Coordinates {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            row: self.row * rhs,
            col: self.col * rhs,
        }
    }
}

impl Ord for Coordinates {
    fn cmp(&self, other: &Self) -> Ordering {
        self.col
            .cmp(&other.col)
            .then_with(|| self.row.cmp(&other.row))
    }
}

impl PartialOrd for Coordinates {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    cost: u64,
    pos: Coordinates,
    prev_dir: Coordinates,
    prev_steps: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    cells: Vec<u64>,
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

    pub fn new(cells: Vec<u64>, row_count: usize, col_count: usize) -> Self {
        assert!(cells.len() == col_count * row_count);
        Self {
            cells,
            size: Coordinates::new(row_count as i64, col_count as i64),
        }
    }

    pub fn get(&self, coords: &Coordinates) -> Option<&u64> {
        match self.coords_to_idx(coords) {
            Some(idx) => Some(&self.cells[idx]),
            None => None,
        }
    }

    pub fn find_min_heat_loss(&self, start: Coordinates, goal: Coordinates) -> u64 {
        let mut visited = HashSet::new();
        let mut stack = BinaryHeap::new();
        stack.push(State {
            cost: 0,
            pos: start,
            prev_dir: Coordinates::new(0, 0),
            prev_steps: 0,
        });
        let mut min_cost = u64::MAX;
        while let Some(State {
            cost,
            pos,
            prev_dir,
            prev_steps,
        }) = stack.pop()
        {
            if visited.contains(&(pos, prev_dir, prev_steps)) {
                continue;
            }
            visited.insert((pos, prev_dir, prev_steps));
            if pos == goal {
                min_cost = min_cost.min(cost);
                break;
            }
            for dir in [
                Coordinates::new(0, -1),
                Coordinates::new(0, 1),
                Coordinates::new(-1, 0),
                Coordinates::new(1, 0),
            ] {
                if dir * -1 == prev_dir {
                    continue;
                }

                let steps = if prev_dir == dir { prev_steps + 1 } else { 1 };

                if steps > 3 {
                    continue;
                }

                let next_pos = pos + dir;
                if let Some(cost_offset) = self.get(&next_pos) {
                    let next_cost = cost + cost_offset;
                    stack.push(State {
                        cost: next_cost,
                        pos: next_pos,
                        prev_dir: dir,
                        prev_steps: steps,
                    });
                }
            }
        }
        min_cost
    }

    pub fn find_min_heat_loss_with_ultra_crucible(
        &self,
        start: Coordinates,
        goal: Coordinates,
    ) -> u64 {
        let mut visited = HashSet::new();
        let mut stack = BinaryHeap::new();
        stack.push(State {
            cost: 0,
            pos: start,
            prev_dir: Coordinates::new(0, 0),
            prev_steps: 0,
        });
        let mut min_cost = u64::MAX;
        while let Some(State {
            cost,
            pos,
            prev_dir,
            prev_steps,
        }) = stack.pop()
        {
            if visited.contains(&(pos, prev_dir, prev_steps)) {
                continue;
            }
            visited.insert((pos, prev_dir, prev_steps));
            if pos == goal && prev_steps >= 4 {
                min_cost = min_cost.min(cost);
                break;
            }

            let next_dirs = if prev_steps < 4 && pos != start {
                vec![prev_dir]
            } else {
                vec![
                    Coordinates::new(0, -1),
                    Coordinates::new(0, 1),
                    Coordinates::new(-1, 0),
                    Coordinates::new(1, 0),
                ]
            };

            for dir in next_dirs {
                if dir * -1 == prev_dir {
                    continue;
                }

                let steps = if prev_dir == dir { prev_steps + 1 } else { 1 };

                if steps > 10 {
                    continue;
                }

                let next_pos = pos + dir;
                if let Some(cost_offset) = self.get(&next_pos) {
                    let next_cost = cost + cost_offset;
                    stack.push(State {
                        cost: next_cost,
                        pos: next_pos,
                        prev_dir: dir,
                        prev_steps: steps,
                    });
                }
            }
        }
        min_cost
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
            grid.cells.append(
                &mut line
                    .chars()
                    .map(|x| x.to_digit(10).unwrap() as u64)
                    .collect(),
            );
        }
        grid
    }
}

fn part_1(grid: &Grid) -> u64 {
    grid.find_min_heat_loss(Coordinates::new(0, 0), grid.size - Coordinates::new(1, 1))
}

fn part_2(grid: &Grid) -> u64 {
    grid.find_min_heat_loss_with_ultra_crucible(
        Coordinates::new(0, 0),
        grid.size - Coordinates::new(1, 1),
    )
}

fn main() {
    use utils::PrintMode;
    utils::run::<_, _>(
        &[
            ("day17/sample_input_1.txt", PrintMode::None),
            ("day17/sample_input_2.txt", PrintMode::None),
            ("day17/input.txt", PrintMode::None),
        ],
        &[part_1, part_2],
    );
}
