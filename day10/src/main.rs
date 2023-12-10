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

#[derive(Debug, Clone, PartialEq, Eq)]
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

    fn idx_to_coords(&self, idx: usize) -> Option<Coordinates> {
        if idx < self.cells.len() {
            Some(Coordinates::new(
                idx as i64 / self.size.col,
                idx as i64 % self.size.col,
            ))
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

    pub fn set(&mut self, coords: &Coordinates, value: T) {
        match self.coords_to_idx(coords) {
            Some(idx) => self.cells[idx] = value,
            None => panic!("Invalid coordinates"),
        }
    }

    pub fn find<P>(&self, predicate: P) -> Option<Coordinates>
    where
        P: FnMut(&T) -> bool,
    {
        match self.cells.iter().position(predicate) {
            Some(idx) => self.idx_to_coords(idx),
            None => None,
        }
    }

    pub fn get_orthogonal_neighbour_coords(&self, coords: &Coordinates) -> Vec<Coordinates> {
        let mut result = Vec::with_capacity(4);
        if coords.row > 0 {
            result.push(*coords + Coordinates::new(-1, 0));
        }
        if coords.row + 1 < self.size.row {
            result.push(*coords + Coordinates::new(1, 0));
        }
        if coords.col > 0 {
            result.push(*coords + Coordinates::new(0, -1));
        }
        if coords.col + 1 < self.size.col {
            result.push(*coords + Coordinates::new(0, 1));
        }
        result
    }

    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    pub fn size(&self) -> Coordinates {
        self.size
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TileKind {
    Start,
    Empty,
    LeftUp,
    LeftRight,
    LeftDown,
    RightUp,
    RightDown,
    UpDown,
}

impl TileKind {
    fn connections(&self) -> Vec<Coordinates> {
        let mut result = Vec::new();
        if self.has_up() {
            result.push(Coordinates::new(-1, 0));
        }
        if self.has_down() {
            result.push(Coordinates::new(1, 0));
        }
        if self.has_left() {
            result.push(Coordinates::new(0, -1));
        }
        if self.has_right() {
            result.push(Coordinates::new(0, 1));
        }
        result
    }

    fn is_start(&self) -> bool {
        *self == Self::Start
    }

    fn has_up(&self) -> bool {
        *self == Self::Start
            || *self == Self::LeftUp
            || *self == Self::RightUp
            || *self == Self::UpDown
    }

    fn has_down(&self) -> bool {
        *self == Self::Start
            || *self == Self::LeftDown
            || *self == Self::RightDown
            || *self == Self::UpDown
    }

    fn has_left(&self) -> bool {
        *self == Self::Start
            || *self == Self::LeftUp
            || *self == Self::LeftRight
            || *self == Self::LeftDown
    }

    fn has_right(&self) -> bool {
        *self == Self::Start
            || *self == Self::RightUp
            || *self == Self::LeftRight
            || *self == Self::RightDown
    }
}

impl From<char> for TileKind {
    fn from(c: char) -> Self {
        match c {
            'S' => TileKind::Start,
            '.' => TileKind::Empty,
            'J' => TileKind::LeftUp,
            '-' => TileKind::LeftRight,
            '7' => TileKind::LeftDown,
            'L' => TileKind::RightUp,
            'F' => TileKind::RightDown,
            '|' => TileKind::UpDown,
            _ => panic!("Unrecognized character"),
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    grid: Grid<TileKind>,
}

impl From<std::fs::File> for Input {
    fn from(file: std::fs::File) -> Self {
        use std::io::{BufRead, BufReader};

        let mut col_count = 0;
        let mut row_count = 0;
        let mut cells = Vec::new();

        for line in BufReader::new(file).lines().map(|x| x.unwrap()) {
            col_count = line.len();
            row_count += 1;
            cells.extend(line.chars().map(|x| TileKind::from(x)));
        }

        Self {
            grid: Grid::new(cells, row_count, col_count),
        }
    }
}

fn find_loop(grid: &Grid<TileKind>) -> Vec<Coordinates> {
    let mut path = Vec::new();

    let start = grid.find(|x| *x == TileKind::Start).unwrap();

    // Find start neighbours actually connecting to the start (should be 2).
    let start_neighbours: Vec<Coordinates> = grid
        .get_orthogonal_neighbour_coords(&start)
        .into_iter()
        .filter(|neighbour| {
            grid.get(neighbour)
                .unwrap()
                .connections()
                .into_iter()
                .any(|connection_offset| *neighbour + connection_offset == start)
        })
        .collect();
    assert!(start_neighbours.len() == 2);

    // Traverse neighbours until we get back to the start, counting how many steps it takes.
    path.push(start);
    path.push(start_neighbours[0]);
    let mut prev_idx = 0;
    let mut curr_idx = 1;
    let mut curr_cell = grid.get(&path[curr_idx]).unwrap();
    while !curr_cell.is_start() {
        let next_candidates: Vec<Coordinates> = curr_cell
            .connections()
            .into_iter()
            .map(|x| path[curr_idx] + x)
            .filter(|x| *x != path[prev_idx] && grid.get(x).is_some())
            .collect();
        assert!(next_candidates.len() == 1);
        path.push(next_candidates[0]);
        prev_idx += 1;
        curr_idx += 1;
        curr_cell = grid.get(&path[curr_idx]).unwrap();
    }

    path
}

fn part_1(input: &Input) -> u64 {
    (find_loop(&input.grid).len() / 2) as u64
}

#[allow(dead_code)]
fn print_grid(grid: &Grid<i64>) {
    for row in 0..grid.size().row {
        let mut s = String::new();
        for col in 0..grid.size().col {
            if let Some(cell) = grid.get(&Coordinates::new(row, col)) {
                match cell {
                    -1 => s.push('X'),
                    0 => s.push('.'),
                    _ => s.push((cell % 10).to_string().chars().next().unwrap()),
                }
            }
        }
        println!("{s}");
    }
}

fn part_2(input: &Input) -> u64 {
    let loop_path = find_loop(&input.grid);

    // Build a larger grid. Each cell is magnified to a 3x3 cell, and walls are filled accordingly.
    // This makes sure that the inner area is actually a single connected area.
    let mut mega_grid = Grid::<i64>::new(
        vec![0; input.grid.cell_count() * 9],
        (input.grid.size().row * 3) as usize,
        (input.grid.size().col * 3) as usize,
    );

    let wall_code = -1;
    for coords in loop_path.iter() {
        let tile_kind = input.grid.get(&coords).unwrap();
        let center = Coordinates::new(coords.row * 3 + 1, coords.col * 3 + 1);

        // Fill the walls based on the original shape.
        // Note that the S cell will have walls in the shape of a cross.
        // This is fine because it will result in a potential connection to all 4 adjacent tiles.
        mega_grid.set(&center, wall_code);
        if tile_kind.has_left() {
            let left = center + Coordinates::new(0, -1);
            mega_grid.set(&left, wall_code);
        }
        if tile_kind.has_right() {
            let right = center + Coordinates::new(0, 1);
            mega_grid.set(&right, wall_code);
        }
        if tile_kind.has_up() {
            let up = center + Coordinates::new(-1, 0);
            mega_grid.set(&up, wall_code);
        }
        if tile_kind.has_down() {
            let down = center + Coordinates::new(1, 0);
            mega_grid.set(&down, wall_code);
        }
    }

    // Use flood fill to identify the inner and the outer area delimited by the walls.
    let mut area_code = 1;
    for row in 0..mega_grid.size().row {
        for col in 0..mega_grid.size().col {
            let coords = Coordinates::new(row, col);
            let tile = mega_grid.get(&coords).unwrap();
            if tile != 0 {
                continue;
            }

            let mut stack = Vec::new();
            stack.push(coords);
            while !stack.is_empty() {
                let coords = stack.pop().unwrap();
                if coords.row >= 0
                    && coords.row < mega_grid.size().row
                    && coords.col >= 0
                    && coords.col < mega_grid.size().col
                {
                    if mega_grid.get(&coords).unwrap() == 0 {
                        mega_grid.set(&coords, area_code);
                        stack.push(coords + Coordinates::new(0, -1));
                        stack.push(coords + Coordinates::new(0, 1));
                        stack.push(coords + Coordinates::new(-1, 0));
                        stack.push(coords + Coordinates::new(1, 0));
                    }
                }
            }

            area_code += 1;
        }
    }

    // By construction, area 1 will always be the outside area, because we start at point 0,0 which is never a wall and always on the outside.
    // There is technically a single situation where this isn't true, in case the start is at the very top left, because by construction then the top left cell in the mega grid is a third area.
    // But it doesn't matter for the inputs of this problem...
    let mut count = 0;
    let label = 2;
    for row in 0..input.grid.size().row {
        for col in 0..input.grid.size().col {
            let coords = Coordinates::new(row * 3 + 1, col * 3 + 1);
            if mega_grid.get(&coords).unwrap() == label {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    use utils::PrintMode;
    utils::run::<_, _>(
        &[
            ("day10/sample_input_1.txt", PrintMode::None),
            ("day10/sample_input_2.txt", PrintMode::None),
            ("day10/sample_input_3.txt", PrintMode::None),
            ("day10/sample_input_4.txt", PrintMode::None),
            ("day10/sample_input_5.txt", PrintMode::None),
            ("day10/input.txt", PrintMode::None),
        ],
        &[part_1, part_2],
    );
}
