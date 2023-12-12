use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpringStatus {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for SpringStatus {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("Invalid spring status"),
        }
    }
}

#[derive(Debug, Clone)]
struct Entry {
    statuses: Vec<SpringStatus>,
    groups: Vec<usize>,
}

impl Entry {
    pub fn expand(&self, count: usize) -> Self {
        let mut statuses = Vec::with_capacity(count * (self.groups.len() + 1) - 1);
        let mut groups = Vec::with_capacity(count * self.groups.len());

        for i in 0..count {
            if i != 0 {
                statuses.push(SpringStatus::Unknown);
            }
            statuses.append(&mut self.statuses.clone());
            groups.append(&mut self.groups.clone());
        }

        Self { statuses, groups }
    }

    pub fn count_arrangements(&self) -> usize {
        self.count_arrangements_rec(&mut HashMap::new(), 0, 0)
    }

    fn count_arrangements_rec(
        &self,
        cache: &mut HashMap<(usize, usize), usize>,
        start_idx: usize,
        group_idx: usize,
    ) -> usize {
        if let Some(count) = cache.get(&(start_idx, group_idx)) {
            return *count;
        }

        let statuses = &self.statuses;
        let group_size = self.groups[group_idx];
        let is_last = (group_idx + 1) == self.groups.len();
        let mut count = 0;

        let mut curr_idx = start_idx;
        while curr_idx + group_size <= statuses.len() {
            let left_is_damaged = curr_idx > 0 && statuses[curr_idx - 1] == SpringStatus::Damaged;

            if left_is_damaged {
                curr_idx += 1;
                continue;
            }

            let next_idx = curr_idx + group_size;
            let it_fits = statuses[curr_idx..next_idx]
                .iter()
                .all(|x| *x != SpringStatus::Operational);

            if it_fits {
                if is_last {
                    let remainder_is_not_damaged = statuses[next_idx..statuses.len()]
                        .iter()
                        .all(|x| *x != SpringStatus::Damaged);
                    if remainder_is_not_damaged {
                        count += 1;
                    }
                } else {
                    if (next_idx + 1) < statuses.len()
                        && statuses[next_idx] != SpringStatus::Damaged
                    {
                        count += self.count_arrangements_rec(cache, next_idx + 1, group_idx + 1);
                    }
                }
            }

            if statuses[curr_idx] == SpringStatus::Damaged {
                break;
            }
            curr_idx += 1;
        }
        cache.insert((start_idx, group_idx), count);
        count
    }
}

#[derive(Debug, Clone)]
struct Input {
    entries: Vec<Entry>,
}

impl From<std::fs::File> for Input {
    fn from(file: std::fs::File) -> Self {
        use std::io::{BufRead, BufReader};

        let mut entries = Vec::new();
        for line in BufReader::new(file).lines().map(Result::unwrap) {
            let (statuses_str, groups_str) = line.split_once(" ").unwrap();
            let statuses = statuses_str
                .chars()
                .map(|x| SpringStatus::from(x))
                .collect();
            let groups = groups_str
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            entries.push(Entry { statuses, groups });
        }
        Self { entries }
    }
}

fn part_1(input: &Input) -> usize {
    input.entries.iter().map(|x| x.count_arrangements()).sum()
}

fn part_2(input: &Input) -> usize {
    input
        .entries
        .iter()
        .map(|x| x.expand(5).count_arrangements())
        .sum()
}

fn main() {
    use utils::PrintMode;
    utils::run::<_, _>(
        &[
            ("day12/sample_input_1.txt", PrintMode::None),
            ("day12/sample_input_2.txt", PrintMode::None),
            ("day12/input.txt", PrintMode::None),
        ],
        &[part_1, part_2],
    );
}
