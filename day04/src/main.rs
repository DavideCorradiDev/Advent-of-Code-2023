#[derive(Debug, Clone)]
struct Card {
    winning_numbers: Vec<u32>,
    owned_numbers: Vec<u32>,
}

impl Card {
    pub fn count_matches(&self) -> usize {
        self.owned_numbers
            .iter()
            .filter(|x| self.winning_numbers.contains(*x))
            .count()
    }

    pub fn calculate_points(&self) -> u32 {
        let matches = self.count_matches() as u32;
        if matches > 0 {
            2u32.pow(matches - 1)
        } else {
            0
        }
    }
}

#[derive(Debug, Clone)]
struct Input {
    cards: Vec<Card>,
}

impl From<std::fs::File> for Input {
    fn from(file: std::fs::File) -> Self {
        use std::io::{BufRead, BufReader};
        let mut cards = Vec::new();
        for line in BufReader::new(file).lines() {
            let line = line.expect("Failed to read line.");
            let (_, numbers) = line.split_once(": ").unwrap();
            let (winning_numbers, owned_numbers) = numbers.split_once(" | ").unwrap();
            let winning_numbers = winning_numbers
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            let owned_numbers = owned_numbers
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            cards.push(Card {
                winning_numbers,
                owned_numbers,
            });
        }
        Self { cards }
    }
}

impl Input {
    pub fn calculate_points(&self) -> u32 {
        self.cards.iter().map(|x| x.calculate_points()).sum()
    }

    pub fn advanced_calculate_points(&self) -> u32 {
        let mut card_copies = vec![1; self.cards.len()];
        for card_idx in 0..self.cards.len() {
            let matches = self.cards[card_idx].count_matches();
            for next_card_idx in (card_idx + 1)..(card_idx + 1 + matches).min(card_copies.len()) {
                card_copies[next_card_idx] += card_copies[card_idx];
            }
        }
        card_copies.iter().sum()
    }
}

fn part_1(input: &Input) -> u32 {
    input.calculate_points()
}

fn part_2(input: &Input) -> u32 {
    input.advanced_calculate_points()
}

fn main() {
    utils::run::<_, _>(
        &["day04/sample_input.txt", "day04/input.txt"],
        &[part_1, part_2],
    );
}
