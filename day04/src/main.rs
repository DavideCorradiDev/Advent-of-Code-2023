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

impl Input {
    pub fn read(file: std::fs::File) -> Self {
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

fn first_part(input: &Input) -> u32 {
    input.calculate_points()
}

fn second_part(input: &Input) -> u32 {
    input.advanced_calculate_points()
}

fn main() -> std::io::Result<()> {
    let input_files = ["sample_input.txt", "input.txt"];
    for input_file in input_files {
        println!("Input: '{input_file}'");
        let input = Input::read(std::fs::File::open("day04/data/".to_owned() + input_file)?);
        let first_part_answer = first_part(&input);
        println!("- First part answer: {first_part_answer}");
        let second_part_answer = second_part(&input);
        println!("- Second part answer: {second_part_answer}");
    }
    Ok(())
}
