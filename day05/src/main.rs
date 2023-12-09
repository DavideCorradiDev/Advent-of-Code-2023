#[derive(Debug, Clone)]
struct Range {
    start: u64,
    count: u64,
}

impl Range {
    fn start(&self) -> u64 {
        self.start
    }

    fn end(&self) -> u64 {
        self.start + self.count
    }

    fn count(&self) -> u64 {
        self.count
    }

    fn intersects(&self, other: &Range) -> Option<Range> {
        let start = self.start().max(other.start());
        let end = self.end().min(other.end());
        if start < end {
            Some(Range {
                start,
                count: end - start,
            })
        } else {
            None
        }
    }

    fn split(&self, other: &Range) -> Option<(Range, Range, Range)> {
        match self.intersects(other) {
            Some(intersection) => {
                let pre_range = Range {
                    start: self.start(),
                    count: intersection.start() - self.start(),
                };
                let post_range = Range {
                    start: intersection.end(),
                    count: self.end() - intersection.end(),
                };
                Some((pre_range, intersection, post_range))
            }
            None => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Ranges {
    dst_start: u64,
    src_start: u64,
    count: u64,
}

impl Ranges {
    fn get_src_range(&self) -> Range {
        Range {
            start: self.src_start,
            count: self.count,
        }
    }

    fn map_range(&self, range: &Range) -> Range {
        Range {
            start: self.dst_start + (range.start() - self.src_start),
            count: range.count(),
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    seeds: Vec<u64>,
    maps: Vec<Vec<Ranges>>,
}

impl From<std::fs::File> for Input {
    fn from(file: std::fs::File) -> Self {
        use std::io::{BufRead, BufReader};

        let mut lines = BufReader::new(file).lines();

        let seeds: Vec<u64> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        let mut maps = Vec::new();

        for line in lines {
            let line = line.unwrap();
            if line.ends_with(":") {
                maps.push(Vec::new());
            } else if !line.is_empty() {
                let (dst_start, src_start, count) = {
                    let values: Vec<u64> = line
                        .split_whitespace()
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect();
                    (values[0], values[1], values[2])
                };
                maps.last_mut().unwrap().push(Ranges {
                    dst_start,
                    src_start,
                    count,
                });
            }
        }

        Self { seeds, maps }
    }
}

fn compute_locations(mut seeds: Vec<u64>, maps: &Vec<Vec<Ranges>>) -> Vec<u64> {
    for map in maps.iter() {
        let prev = seeds.clone();
        for ranges in map.iter() {
            for i in 0..prev.len() {
                if prev[i] >= ranges.src_start && prev[i] < ranges.src_start + ranges.count {
                    seeds[i] = ranges.dst_start + (prev[i] - ranges.src_start);
                }
            }
        }
    }
    seeds
}

fn part_1(input: &Input) -> u64 {
    let locations = compute_locations(input.seeds.clone(), &input.maps);
    *locations.iter().min().unwrap()
}

fn compute_location_ranges(mut seeds: Vec<Range>, maps: &Vec<Vec<Ranges>>) -> Vec<Range> {
    for map in maps.iter() {
        let mut added_ranges = Vec::new();
        for ranges in map.iter() {
            let src_range = ranges.get_src_range();
            let mut split_ranges = Vec::new();
            for i in 0..seeds.len() {
                if let Some((pre_range, intersection, post_range)) = seeds[i].split(&src_range) {
                    split_ranges.push((i, pre_range, post_range));
                    added_ranges.push(ranges.map_range(&intersection));
                }
            }
            let mut removed_count = 0;
            for (i, pre_range, post_range) in split_ranges {
                let idx = i - removed_count;
                if pre_range.count() > 0 {
                    seeds[idx] = pre_range;
                    if post_range.count() > 0 {
                        seeds.push(post_range);
                    }
                } else if post_range.count() > 0 {
                    seeds[idx] = post_range;
                } else {
                    seeds.remove(idx);
                    removed_count += 1;
                }
            }
        }
        seeds.append(&mut added_ranges);
    }
    seeds
}

fn part_2(input: &Input) -> u64 {
    let mut seed_ranges = Vec::new();
    for i in (0..input.seeds.len()).step_by(2) {
        seed_ranges.push(Range {
            start: input.seeds[i],
            count: input.seeds[i + 1],
        })
    }
    let location_ranges = compute_location_ranges(seed_ranges, &input.maps);
    location_ranges
        .iter()
        .min_by(|x, y| x.start.cmp(&y.start))
        .unwrap()
        .start
}

fn main() {
    use utils::PrintMode;
    utils::run::<_, _>(
        &[
            ("day05/sample_input.txt", PrintMode::None),
            ("day05/input.txt", PrintMode::None),
        ],
        &[part_1, part_2],
    );
}
