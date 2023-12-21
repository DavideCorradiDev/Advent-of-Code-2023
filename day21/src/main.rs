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
struct Grid<T> {
    cells: Vec<T>,
    size: Vec2i,
}

impl<T> Grid<T>
where
    T: From<char> + PartialEq + Eq,
{
    fn coords_to_idx(&self, coords: &Vec2i) -> Option<usize> {
        if coords.x >= 0 && coords.x < self.size.x && coords.y >= 0 && coords.y < self.size.y {
            Some((self.size.x * coords.y + coords.x) as usize)
        } else {
            None
        }
    }

    fn idx_to_coords(&self, idx: usize) -> Option<Vec2i> {
        if idx < self.cells.len() {
            Some(Vec2i::new(
                idx as i64 % self.size.x,
                idx as i64 / self.size.x,
            ))
        } else {
            None
        }
    }

    pub fn new(cells: Vec<T>, size: Vec2i) -> Self {
        assert!(cells.len() as i64 == size.x * size.y);
        Self { cells, size }
    }

    pub fn get(&self, coords: &Vec2i) -> Option<&T> {
        match self.coords_to_idx(coords) {
            Some(idx) => Some(&self.cells[idx]),
            None => None,
        }
    }

    pub fn get_mut(&mut self, coords: &Vec2i) -> Option<&mut T> {
        match self.coords_to_idx(coords) {
            Some(idx) => Some(&mut self.cells[idx]),
            None => None,
        }
    }

    pub fn calc_distances(&self, max_distance: u64) -> Grid<u64> {
        let start_idx = self.cells.iter().position(|x| *x == T::from('S')).unwrap();
        let start = self.idx_to_coords(start_idx).unwrap();

        let mut distances = Grid::new(vec![u64::MAX; self.cells.len()], self.size);
        *distances.get_mut(&start).unwrap() = 0;

        let mut stack = Vec::new();
        stack.push(start);
        while let Some(pos) = stack.pop() {
            let distance = distances.get(&pos).unwrap() + 1;
            if distance <= max_distance {
                for offset in [
                    Vec2i::new(-1, 0),
                    Vec2i::new(1, 0),
                    Vec2i::new(0, -1),
                    Vec2i::new(0, 1),
                ] {
                    let next_pos = pos + offset;
                    if let Some(next_cell) = self.get(&next_pos) {
                        if *next_cell != T::from('#') {
                            let next_dist = distances.get_mut(&next_pos).unwrap();
                        if distance < *next_dist {
                            *next_dist = distance;
                            stack.push(next_pos);
                        }
                        }
                    }
                }
            }
        }

        distances
    }

    pub fn count_cells_reachable_by_steps(&self, steps: u64) -> usize {
        self.calc_distances(steps)
            .cells
            .iter()
            .filter(|dist| **dist < u64::MAX && **dist % 2 == steps % 2)
            .count()
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    T: From<char> + PartialEq + Eq + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                write!(f, "{}", self.get(&Vec2i::new(x, y)).unwrap())?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<T> From<std::fs::File> for Grid<T>
where
    T: From<char>,
{
    fn from(file: std::fs::File) -> Self {
        use std::io::{BufRead, BufReader};
        let mut cells = Vec::new();
        let mut size = Vec2i::new(0, 0);
        for line in BufReader::new(file).lines().map(Result::unwrap) {
            size.x = line.len() as i64;
            size.y += 1;
            cells.append(&mut line.chars().map(T::from).collect());
        }
        Self { cells, size }
    }
}

fn part_1(grid: &Grid<char>) -> Vec<usize> {
    [6, 64].iter().map(|x| grid.count_cells_reachable_by_steps(*x)).collect()
}

fn part_2(grid: &Grid<char>) -> Vec<usize> {
    Vec::new()
}

fn main() {
    use utils::PrintMode;
    utils::run::<_, _>(
        &[
            ("day21/sample_input.txt", PrintMode::None),
            ("day21/input.txt", PrintMode::None),
        ],
        &[part_1, part_2],
    );
}
