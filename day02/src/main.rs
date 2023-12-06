use std::fs::File;

#[derive(Debug, Clone, Copy)]
struct CubeSet {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl CubeSet {
    pub fn new() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    pub fn power(&self) -> u32 {
        self.r * self.g * self.b
    }
}

impl From<&str> for CubeSet {
    fn from(value: &str) -> Self {
        let mut ans = CubeSet::new();
        for color_str in value.split(", ") {
            let (num, color) = color_str.split_once(" ").unwrap();
            match color {
                "red" => ans.r = num.parse::<u32>().unwrap(),
                "green" => ans.g = num.parse::<u32>().unwrap(),
                "blue" => ans.b = num.parse::<u32>().unwrap(),
                _ => panic!("Unsupported color"),
            }
        }
        ans
    }
}

fn read_input(file: File) -> Vec<Vec<CubeSet>> {
    use std::io::{BufRead, BufReader};

    let mut ans = Vec::new();

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let (_, game_str) = line.split_once(": ").unwrap();
        let mut game = Vec::new();
        for iteration_str in game_str.split("; ") {
            game.push(CubeSet::from(iteration_str));
        }
        ans.push(game);
    }

    ans
}

fn is_valid_game(game: &Vec<CubeSet>, all_cubes: &CubeSet) -> bool {
    for game_iter in game {
        if game_iter.r > all_cubes.r || game_iter.b > all_cubes.b || game_iter.g > all_cubes.g {
            return false;
        }
    }
    true
}

fn first_part() {
    let input =
        read_input(File::open("day02/data/sample_input.txt").expect("Couldn't open input file"));
    let all_cubes = CubeSet {
        r: 12,
        g: 13,
        b: 14,
    };
    let mut ans = 0;
    for (idx, game) in input.iter().enumerate() {
        if is_valid_game(&game, &all_cubes) {
            ans += idx + 1;
        }
    }
    println!("The answer is '{ans}'.");
}

fn compute_minimum_viable_cube_set(game: &Vec<CubeSet>) -> CubeSet {
    use std::cmp::max;

    let mut ans = CubeSet::new();
    for game_iteration in game.iter() {
        ans.r = max(ans.r, game_iteration.r);
        ans.g = max(ans.g, game_iteration.g);
        ans.b = max(ans.b, game_iteration.b);
    }
    ans
}

fn second_part() {
    let input = read_input(File::open("day02/data/input.txt").expect("Couldn't open input file"));
    let minimum_viable_cube_sets: Vec<CubeSet> = input
        .iter()
        .map(|x| compute_minimum_viable_cube_set(x))
        .collect();
    let powers: Vec<u32> = minimum_viable_cube_sets.iter().map(|x| x.power()).collect();
    let ans: u32 = powers.iter().sum();
    println!("The answer is '{ans}'.");
}

fn main() {
    first_part();
    second_part();
}
