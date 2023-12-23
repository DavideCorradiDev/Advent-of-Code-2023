use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    T: PartialEq + Eq,
{
    fn coords_to_idx(&self, coords: &Vec2i) -> Option<usize> {
        if coords.x >= 0 && coords.x < self.size.x && coords.y >= 0 && coords.y < self.size.y {
            Some((self.size.x * coords.y + coords.x) as usize)
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Graph {
    edges: Vec<Vec<(usize, usize)>>,
    start_node: usize,
    goal_node: usize,
}

impl Graph {
    fn longest_path(&self) -> usize {
        let mut distances = vec![0; self.edges.len()];
        let mut stack = Vec::new();
        stack.push((self.start_node, 0));
        while let Some((node, distance)) = stack.pop() {
            for (next_node, edge_cost) in &self.edges[node] {
                let next_distance = distance + edge_cost;
                if next_distance > distances[*next_node] {
                    distances[*next_node] = next_distance;
                    stack.push((*next_node, next_distance));
                }
            }
        }
        distances[self.goal_node]
    }

    fn make_cyclic(&mut self) {
        let edges_clone = self.edges.clone();
        for src in 0..edges_clone.len() {
            for (dest, cost) in edges_clone[src].iter() {
                self.edges[*dest].push((src, *cost));
            }
        }
    }

    fn cyclic_longest_path(&self) -> usize {
        let mut max_distance = 0;
        let mut stack = Vec::new();
        stack.push((vec![self.start_node], 0));
        while let Some((path, distance)) = stack.pop() {
            let node = path.last().unwrap();
            for (next_node, edge_cost) in &self.edges[*node] {
                if *next_node == self.goal_node {
                    max_distance = max_distance.max(distance + edge_cost);
                } else {
                    if !path.iter().any(|x| x == next_node) {
                        let mut path = path.clone();
                        path.push(*next_node);
                        stack.push((path, distance + edge_cost));
                    }
                }
            }
        }
        max_distance
    }
}

impl From<Grid<char>> for Graph {
    fn from(grid: Grid<char>) -> Self {
        let slope_to_dir = |slope| match slope {
            '<' => Vec2i::new(-1, 0),
            '>' => Vec2i::new(1, 0),
            '^' => Vec2i::new(0, -1),
            'v' => Vec2i::new(0, 1),
            _ => panic!("Invalid slope"),
        };

        let mut nodes = HashMap::new();
        let mut edges = Vec::new();
        let goal_pos = Vec2i::new(grid.size.x - 2, grid.size.y - 1);
        let mut goal_node = 0;
        let mut visited = Grid::<bool>::new(vec![false; grid.cells.len()], grid.size.clone());

        let mut stack = Vec::new();
        let mut node_counter = 0;

        nodes.insert(Vec2i::new(1, 0), node_counter);
        stack.push((node_counter, Vec2i::new(1, 0), 0));
        node_counter += 1;

        while let Some((node_idx, pos, distance)) = stack.pop() {
            if *visited.get(&pos).unwrap() {
                continue;
            }
            *visited.get_mut(&pos).unwrap() = true;

            if edges.len() <= node_idx {
                edges.resize(node_idx + 1, Vec::new());
            }

            if pos == goal_pos {
                goal_node = node_counter;
                node_counter += 1;
                edges[node_idx].push((goal_node, distance));
                nodes.insert(pos, goal_node);
                continue;
            }

            for offset in [
                Vec2i::new(-1, 0),
                Vec2i::new(1, 0),
                Vec2i::new(0, -1),
                Vec2i::new(0, 1),
            ] {
                let next_pos = pos + offset;
                if let Some(next_cell) = grid.get(&next_pos) {
                    match next_cell {
                        '#' => (),
                        '.' => stack.push((node_idx, next_pos, distance + 1)),
                        '<' | '>' | '^' | 'v' => {
                            let next_node_pos = next_pos + slope_to_dir(*next_cell);
                            if next_node_pos != pos {
                                if distance == 0 {
                                    stack.push((node_idx, next_pos, distance + 1));
                                } else {
                                    let next_node = {
                                        match nodes.get(&next_node_pos) {
                                            Some(node) => node.clone(),
                                            None => {
                                                let node = node_counter;
                                                node_counter += 1;
                                                node
                                            }
                                        }
                                    };

                                    nodes.insert(next_node_pos, next_node);
                                    edges[node_idx].push((next_node, distance + 2));
                                    stack.push((next_node, next_node_pos, 0));
                                }
                            }
                        }
                        _ => panic!("invalid cell"),
                    }
                }
            }
        }
        Self {
            edges,
            start_node: 0,
            goal_node,
        }
    }
}

fn part_1(grid: &Grid<char>) -> usize {
    Graph::from(grid.clone()).longest_path()
}

fn part_2(grid: &Grid<char>) -> usize {
    let mut graph = Graph::from(grid.clone());
    graph.make_cyclic();
    graph.cyclic_longest_path()
}

fn main() {
    use utils::PrintMode;
    utils::run::<_, _>(
        &[
            ("day23/sample_input.txt", PrintMode::None),
            ("day23/input.txt", PrintMode::None),
        ],
        &[part_1, part_2],
    );
}
