#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: [u64; 5],
    bid: u64,
}

impl Hand {
    fn count_cards(&self) -> Vec<u64> {
        let mut joker_count = 0;
        let mut counter: Vec<(u64, u64)> = Vec::new();
        for card in self.cards {
            if card == 1 {
                joker_count += 1;
            } else if let Some((_, count)) = counter.iter_mut().find(|x| x.0 == card) {
                *count += 1;
            } else {
                counter.push((card, 1));
            }
        }
        if counter.is_empty() {
            vec![joker_count]
        }
        else {
            let mut counter: Vec<u64> = counter.iter().map(|x| x.1).collect();
            counter.sort_by(|x, y| y.cmp(x));
            counter[0] += joker_count;
            counter
        }
    }

    fn kind(&self) -> HandKind {
        let counter = self.count_cards();
        match counter.len() {
            1 => HandKind::FiveOfAKind,
            2 => {
                if counter[0] == 4 {
                    HandKind::FourOfAKind
                } else {
                    HandKind::FullHouse
                }
            }
            3 => {
                if counter[0] == 3 {
                    HandKind::ThreeOfAKind
                } else {
                    HandKind::TwoPair
                }
            }
            4 => HandKind::OnePair,
            _ => HandKind::HighCard,
        }
    }

    fn replace_jacks_with_jokers(&mut self) {
        for card in self.cards.iter_mut() {
            if *card == 11 {
                *card = 1
            }
        }
    }
}

impl From<String> for Hand {
    fn from(s: String) -> Self {
        let (cards_str, bid_str) = s.split_once(" ").unwrap();

        let cards = cards_str
            .chars()
            .map(|x| match x {
                '2'..='9' => x.to_digit(10).unwrap() as u64,
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("Unrecognized character"),
            })
            .collect::<Vec<u64>>()
            .try_into()
            .unwrap();

        let bid = bid_str.parse::<u64>().unwrap();

        Self { cards, bid }
    }
}

impl std::cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.kind().cmp(&other.kind()) {
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => {
                for i in 0..self.cards.len() {
                    match self.cards[i].cmp(&other.cards[i]) {
                        std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                        std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                        _ => (),
                    }
                }
                std::cmp::Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct Input {
    hands: Vec<Hand>,
}

impl Input {
    fn compute_score(&mut self) -> u64 {
        self.hands.sort();
        self.hands
            .iter()
            .enumerate()
            .map(|(idx, x)| (idx + 1) as u64 * x.bid)
            .sum()
    }

    fn replace_jacks_with_jokers(&mut self) {
        for hand in self.hands.iter_mut() {
            hand.replace_jacks_with_jokers();
        }
    }
}

impl From<std::fs::File> for Input {
    fn from(file: std::fs::File) -> Self {
        use std::io::{BufRead, BufReader};

        let hands = BufReader::new(file)
            .lines()
            .map(Result::unwrap)
            .map(|x| Hand::from(x))
            .collect();

        Self { hands }
    }
}

fn part_1(input: &Input) -> u64 {
    input.clone().compute_score()
}

fn part_2(input: &Input) -> u64 {
    let mut input = input.clone();
    input.replace_jacks_with_jokers();
    input.compute_score()
}

fn main() {
    use utils::PrintMode;
    utils::run::<_, _>(
        &[("day07/sample_input.txt", PrintMode::None), ("day07/input.txt", PrintMode::None)],
        &[part_1, part_2],
    );
}
