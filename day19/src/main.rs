use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Part {
    properties: HashMap<char, u64>,
}

impl Part {
    fn total_rating(&self) -> u64 {
        self.properties.iter().map(|(_, val)| val).sum()
    }

    fn is_accepted(&self, worflows: &HashMap<String, Vec<Rule>>) -> bool {
        let mut curr_workflow = "in";
        loop {
            let outcome = &worflows
                .get(curr_workflow)
                .unwrap()
                .iter()
                .find(|rule| rule.matches(self))
                .unwrap()
                .outcome;
            match outcome {
                Outcome::Accept => return true,
                Outcome::Refuse => return false,
                Outcome::GoTo(workflow) => {
                    curr_workflow = workflow;
                }
            }
        }
    }
}

impl From<&str> for Part {
    fn from(s: &str) -> Self {
        let properties = s[1..s.len() - 1]
            .split(",")
            .into_iter()
            .map(|x| {
                let (name, val) = x.split_once("=").unwrap();
                (name.chars().next().unwrap(), val.parse::<u64>().unwrap())
            })
            .collect();
        Self { properties }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Outcome {
    GoTo(String),
    Accept,
    Refuse,
}

impl From<&str> for Outcome {
    fn from(s: &str) -> Self {
        match s {
            "A" => Outcome::Accept,
            "R" => Outcome::Refuse,
            _ => Outcome::GoTo(String::from(s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Comparison {
    property: char,
    ordering: std::cmp::Ordering,
    value: u64,
}

impl Comparison {
    fn matches(&self, part: &Part) -> bool {
        part.properties
            .get(&self.property)
            .unwrap()
            .cmp(&self.value)
            == self.ordering
    }
}

impl From<&str> for Comparison {
    fn from(s: &str) -> Self {
        let substrings: Vec<&str> = s.split_inclusive(|x| x == '<' || x == '>').collect();
        let mut chars = substrings[0].chars();
        let property = chars.next().unwrap();
        let ordering = match chars.next().unwrap() {
            '<' => std::cmp::Ordering::Less,
            '>' => std::cmp::Ordering::Greater,
            _ => panic!("Invalid operator"),
        };
        let value = substrings[1].parse::<u64>().unwrap();

        Self {
            property,
            ordering,
            value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule {
    comparison: Option<Comparison>,
    outcome: Outcome,
}

impl Rule {
    fn matches(&self, part: &Part) -> bool {
        match &self.comparison {
            Some(comparison) => comparison.matches(part),
            None => true,
        }
    }
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        match s.split_once(":") {
            Some((comparison, outcome)) => Self {
                comparison: Some(Comparison::from(comparison)),
                outcome: Outcome::from(outcome),
            },
            None => Self {
                comparison: None,
                outcome: Outcome::from(s),
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    workflows: HashMap<String, Vec<Rule>>,
    parts: Vec<Part>,
}

impl From<std::fs::File> for Input {
    fn from(file: std::fs::File) -> Self {
        use std::io::{BufRead, BufReader};
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let mut workflows = HashMap::new();
        while let Some(line) = lines.next() {
            let line = line.unwrap();
            if line.is_empty() {
                break;
            }
            let (worflow_name, rules) = line.split_once("{").unwrap();
            let workflow_rules = rules[0..rules.len() - 1]
                .split(",")
                .map(|x| Rule::from(x))
                .collect();
            workflows.insert(String::from(worflow_name), workflow_rules);
        }

        let mut parts = Vec::new();
        while let Some(line) = lines.next() {
            let line = line.unwrap();
            parts.push(Part::from(&line[..]));
        }

        Self { workflows, parts }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Chain {
    exclude: Vec<Comparison>,
    include: Vec<Comparison>,
}

fn find_acceptance_chains(workflows: &HashMap<String, Vec<Rule>>) -> Vec<Chain> {
    let mut chains = Vec::new();
    let mut stack = Vec::new();

    stack.push((
        "in",
        Chain {
            exclude: Vec::new(),
            include: Vec::new(),
        },
    ));
    while let Some((workflow, chain)) = stack.pop() {
        let rules = workflows.get(workflow).unwrap();
        let mut chain = chain.clone();
        for rule in rules {
            let mut next_chain = chain.clone();
            if let Some(comparison) = &rule.comparison {
                next_chain.include.push(comparison.clone());
                chain.exclude.push(comparison.clone());
            }
            match &rule.outcome {
                Outcome::Accept => chains.push(next_chain),
                Outcome::Refuse => (),
                Outcome::GoTo(workflow) => stack.push((workflow, next_chain)),
            }
        }
    }

    chains
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HyperCube {
    min: [u64; 4],
    max: [u64; 4],
}

impl HyperCube {
    fn volume(&self) -> u64 {
        let mut volume = 1;
        for i in 0..self.min.len() {
            volume *= self.max[i] - self.min[i];
        }
        volume
    }
}

impl From<Chain> for HyperCube {
    fn from(chain: Chain) -> Self {
        let mut min = [1; 4];
        let mut max = [4001; 4];

        for Comparison {
            property,
            ordering,
            value,
        } in chain.include
        {
            let idx = match property {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                's' => 3,
                _ => panic!("Invalid property"),
            };
            match ordering {
                std::cmp::Ordering::Greater => min[idx] = min[idx].max(value + 1),
                std::cmp::Ordering::Less => max[idx] = max[idx].min(value),
                _ => panic!("Invalid ordering"),
            }
        }

        for Comparison {
            property,
            ordering,
            value,
        } in chain.exclude
        {
            let idx = match property {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                's' => 3,
                _ => panic!("Invalid property"),
            };
            match ordering {
                std::cmp::Ordering::Less => min[idx] = min[idx].max(value),
                std::cmp::Ordering::Greater => max[idx] = max[idx].min(value + 1),
                _ => panic!("Invalid ordering"),
            }
        }

        Self { min, max }
    }
}

fn part_1(input: &Input) -> u64 {
    input
        .parts
        .iter()
        .filter(|part| part.is_accepted(&input.workflows))
        .map(|part| part.total_rating())
        .sum()
}

fn part_2(input: &Input) -> u64 {
    find_acceptance_chains(&input.workflows)
        .into_iter()
        .map(HyperCube::from)
        .map(|x| x.volume())
        .sum()
}

fn main() {
    use utils::PrintMode;
    utils::run::<_, _>(
        &[
            ("day19/sample_input.txt", PrintMode::None),
            ("day19/input.txt", PrintMode::None),
        ],
        &[part_1, part_2],
    );
}
