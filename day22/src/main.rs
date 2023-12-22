use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec3i {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3i {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

impl std::ops::AddAssign for Vec3i {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Add for Vec3i {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl std::ops::SubAssign for Vec3i {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::Sub for Vec3i {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl std::ops::MulAssign<i64> for Vec3i {
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl std::ops::Mul<i64> for Vec3i {
    type Output = Self;
    fn mul(mut self, rhs: i64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl std::ops::Mul<Vec3i> for i64 {
    type Output = Vec3i;
    fn mul(self, rhs: Vec3i) -> Self::Output {
        rhs * self
    }
}

impl From<&str> for Vec3i {
    fn from(s: &str) -> Self {
        let coords: Vec<i64> = s
            .split(",")
            .map(|x| i64::from_str_radix(x, 10).unwrap())
            .collect();
        Vec3i::new(coords[0], coords[1], coords[2])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Brick {
    min: Vec3i,
    max: Vec3i,
}

impl Brick {
    fn new(min: Vec3i, max: Vec3i) -> Self {
        assert!(max.x >= min.x);
        assert!(max.y >= min.y);
        assert!(max.z >= min.z);
        Self { min, max }
    }

    fn overlaps_xy(&self, rhs: &Brick) -> bool {
        self.max.x >= rhs.min.x
            && rhs.max.x >= self.min.x
            && self.max.y >= rhs.min.y
            && rhs.max.y >= self.min.y
    }

    fn set_z(&mut self, z: i64) {
        let height = self.max.z - self.min.z;
        self.min.z = z;
        self.max.z = self.min.z + height;
    }
}

#[derive(Debug, Clone)]
struct Grid {
    // Assumed sorted on from lower to higher min z coordinate.
    bricks: Vec<Brick>,
}

impl Grid {
    fn sort(&mut self) {
        self.bricks.sort_by(|lhs, rhs| lhs.min.z.cmp(&rhs.min.z));
    }

    fn collapse(&mut self) {
        self.sort();
        for idx_0 in 0..self.bricks.len() {
            let orig_z = self.bricks[idx_0].min.z;
            self.bricks[idx_0].set_z(1);
            for idx_1 in 1..idx_0 + 1 {
                let idx_1 = idx_0 - idx_1;
                let z = self.bricks[idx_1].max.z;
                if z < orig_z
                    && z >= self.bricks[idx_0].min.z
                    && self.bricks[idx_0].overlaps_xy(&self.bricks[idx_1])
                {
                    self.bricks[idx_0].set_z(z + 1);
                }
            }
        }
        self.sort();
    }

    fn find_supporting_bricks(&self, idx_0: usize) -> Vec<usize> {
        let mut ans = Vec::new();
        let brick = &self.bricks[idx_0];
        for idx_1 in 1..idx_0 + 1 {
            let idx_1 = idx_0 - idx_1;
            let other_brick = &self.bricks[idx_1];
            if other_brick.max.z + 1 == brick.min.z && other_brick.overlaps_xy(brick) {
                ans.push(idx_1);
            }
        }
        ans
    }

    fn find_supported_bricks(&self, idx_0: usize) -> Vec<usize> {
        let mut ans = Vec::new();
        let brick = &self.bricks[idx_0];
        for idx_1 in idx_0 + 1..self.bricks.len() {
            let other_brick = &self.bricks[idx_1];
            if brick.max.z + 1 == other_brick.min.z && other_brick.overlaps_xy(brick) {
                ans.push(idx_1);
            }
        }
        ans
    }

    fn find_required_bricks(&self) -> Vec<bool> {
        let mut ans = vec![false; self.bricks.len()];
        for i in 0..self.bricks.len() {
            let supporting_blocks = self.find_supporting_bricks(i);
            if supporting_blocks.len() == 1 {
                ans[supporting_blocks[0]] = true;
            }
        }
        ans
    }

    fn count_destroyable_bricks(&self) -> usize {
        let required_bricks = self.find_required_bricks();
        required_bricks.iter().filter(|x| **x == false).count()
    }

    fn count_chain_reactions(&self) -> usize {
        let required_bricks: Vec<usize> = self
            .find_required_bricks()
            .iter()
            .enumerate()
            .filter(|(_, x)| **x == true)
            .map(|(idx, _)| idx)
            .collect();

        let supported_bricks: Vec<Vec<usize>> = self
            .bricks
            .iter()
            .enumerate()
            .map(|(idx, _)| self.find_supported_bricks(idx))
            .collect();

        let supporting_bricks: Vec<Vec<usize>> = self
            .bricks
            .iter()
            .enumerate()
            .map(|(idx, _)| self.find_supporting_bricks(idx))
            .collect();

        let mut ans = 0;
        for idx in required_bricks.into_iter() {
            let mut removed_blocks = HashSet::new();
            let mut stack = Vec::new();
            stack.push(idx);
            while let Some(idx) = stack.pop() {
                if removed_blocks.insert(idx) {
                    for other_idx in supported_bricks[idx].iter() {
                        if supporting_bricks[*other_idx]
                            .iter()
                            .all(|x| removed_blocks.contains(x))
                        {
                            stack.push(*other_idx);
                        }
                    }
                }
            }
            ans += removed_blocks.len() - 1;
        }
        ans
    }
}

impl From<std::fs::File> for Grid {
    fn from(file: std::fs::File) -> Self {
        use std::io::{BufRead, BufReader};
        let bricks: Vec<Brick> = BufReader::new(file)
            .lines()
            .map(Result::unwrap)
            .map(|line| {
                let (min, max) = line.split_once("~").unwrap();
                let min = Vec3i::from(min);
                let max = Vec3i::from(max);
                Brick::new(min, max)
            })
            .collect();
        Self { bricks }
    }
}

fn part_1(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    grid.collapse();
    grid.count_destroyable_bricks()
}

fn part_2(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    grid.collapse();
    grid.count_chain_reactions()
}

fn main() {
    use utils::PrintMode;
    utils::run::<_, _>(
        &[
            ("day22/sample_input.txt", PrintMode::None),
            ("day22/input.txt", PrintMode::None),
        ],
        &[part_1, part_2],
    );
}
